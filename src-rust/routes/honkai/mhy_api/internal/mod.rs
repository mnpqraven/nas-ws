use self::{
    categorizing::{CharacterSkillTree, DbCharacter},
    constants::{CHARACTER_REMOTE, CHARACTER_SKILL_TREE_REMOTE},
};
use crate::{handler::error::WorkerError, routes::honkai::mhy_api::internal::impls::DbData};
use anyhow::Result;
use axum::{extract::Path, Json};
use serde::de::DeserializeOwned;
use std::{collections::HashMap, fs, sync::Arc};
use tracing::{info, instrument};

/// holds internal types for mhy's DB
// TODO: avoid conflicting type names with super::types
pub mod categorizing;
pub mod constants;
pub mod impls;

// NOTE: url fetching
pub async fn get_character_list() -> Result<Arc<[DbCharacter]>> {
    let now = std::time::Instant::now();
    let pathname = "/tmp/characters.json";
    let data: Vec<DbCharacter> = match std::path::Path::new(pathname).exists() {
        true => {
            let t = fs::read_to_string(pathname)?;
            serde_json::from_str(&t)?
        }
        false => {
            let res_str: String = reqwest::get(CHARACTER_REMOTE).await?.text().await?;
            let map: HashMap<String, DbCharacter> = serde_json::from_str(&res_str)?;
            map.into_values().collect()
        }
    };
    info!("get_character_list {:?}", now.elapsed());
    Ok(data.into())
}

pub async fn character_by_id(Path(id): Path<u32>) -> Result<Json<DbCharacter>, WorkerError> {
    let now = std::time::Instant::now();

    let characters = DbCharacter::read().await?;

    let db_char = characters.get(&id.to_string()).cloned();

    info!("{:?}", now.elapsed());
    match db_char {
        Some(t) => Ok(Json(t)),
        None => Err(WorkerError::EmptyBody),
    }
}

// TODO: change params (?)
#[instrument(ret)]
pub async fn skill_tree_by_character_id(
    Path(char_id): Path<u32>,
) -> Result<Json<Vec<CharacterSkillTree>>, WorkerError> {
    let Json(character) = character_by_id(Path(char_id)).await.unwrap();

    let binding = reqwest::get(CHARACTER_SKILL_TREE_REMOTE)
        .await?
        .text()
        .await?;
    let db_list: HashMap<String, CharacterSkillTree> = serde_json::from_str(&binding).unwrap();

    let res: Arc<[CharacterSkillTree]> = character
        .skill_trees
        .iter()
        .map(|key| db_list.get(key).cloned().unwrap())
        .collect();
    let list = res.to_vec();
    Ok(Json(list))
}

/// TODO: needs a lighter KV map
/// TODO: needs a faster lookup method
/// NOTE: probably can deprecate this
/// try returning hashmap directly and get keys from there
pub async fn get_db_list<T>(filename: &str, fallback_url: &str) -> Result<Arc<[T]>>
where
    T: DeserializeOwned,
{
    let data: Vec<T> = match std::path::Path::new(filename).exists() {
        true => {
            info!("reading from file");
            let t = fs::read_to_string(filename)?;
            serde_json::from_str(&t)?
        }
        false => {
            info!("fetching from fallback url");
            let res_str: String = reqwest::get(fallback_url).await?.text().await?;
            let map: HashMap<String, T> = serde_json::from_str(&res_str)?;
            map.into_values().collect()
        }
    };
    Ok(data.into())
}

#[cfg(test)]
mod tests {
    use super::get_character_list;
    use crate::routes::honkai::mhy_api::internal::{
        categorizing::DbCharacterSkill, get_db_list, impls::Queryable,
    };

    #[tokio::test]
    async fn calling() {
        let list = get_character_list().await.unwrap();
        let kafka = list.iter().find(|e| e.name.eq("Luocha")).unwrap();

        let skill_db = get_db_list::<DbCharacterSkill>("character_skills.json", "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_new/en/character_skills.json").await.unwrap();

        let skill_ids = kafka.skill_ids();
        let t = skill_db.find_many(skill_ids);

        let to_test = &t.iter().find(|e| e.id == 120304).unwrap();
        dbg!(&to_test.desc);
        dbg!(&to_test.params[0]);

        dbg!(&to_test.split_description());

        // let right = vec![
        //     "Deals Lightning DMG equal to ",
        //     " of Kafka's ATK to all enemies, with a ",
        //     " base chance for enemies hit to become Shocked and immediately take DMG equal to ",
        //     " of the DoT. Shock lasts for ",
        //     " turn(s).\nWhile Shocked, enemies receive Lightning DoT equal to ",
        //     " of Kafka's ATK at the beginning of each turn.",
        // ];
        // assert_eq!(
        //     to_test
        //         .split_description()
        //         .iter()
        //         .map(|e| e.to_string())
        //         .collect::<Vec<String>>(),
        //     right
        // );
    }
}
