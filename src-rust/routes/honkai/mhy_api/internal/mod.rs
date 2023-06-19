use self::{categorizing::DbCharacter, constants::CHARACTER_DICT};
use anyhow::Result;
use serde::de::DeserializeOwned;
use std::{collections::HashMap, fs, path::Path, sync::Arc};
use tracing::{debug, info};

/// holds internal types for mhy's DB
// TODO: avoid conflicting type names with super::types
pub mod categorizing;
pub mod constants;
pub mod impls;
#[cfg(test)]
mod runnables;

// NOTE: url fetching
pub async fn get_character_list() -> Result<Arc<[DbCharacter]>> {
    let now = std::time::Instant::now();
    let pathname = "/tmp/characters.json";
    let data: Vec<DbCharacter> = match Path::new(pathname).exists() {
        true => {
            let t = fs::read_to_string(pathname)?;
            serde_json::from_str(&t)?
        }
        false => {
            let res_str: String = reqwest::get(CHARACTER_DICT).await?.text().await?;
            let map: HashMap<String, DbCharacter> = serde_json::from_str(&res_str)?;
            map.into_values().collect()
        }
    };
    info!("get_character_list {:?}", now.elapsed());
    Ok(data.into())
}

pub async fn get_db_list<T>(filename: &str, fallback_url: &str) -> Result<Arc<[T]>>
where
    T: DeserializeOwned,
{
    let data: Vec<T> = match Path::new(filename).exists() {
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

/// attempts to get character data from github and writes to /tmp
/// @returns: tuple of whether the file already exists and whether writing has succeeded
#[allow(dead_code)]
pub async fn write_character_db() -> Result<(bool, bool)> {
    let res_str: String = reqwest::get(CHARACTER_DICT).await?.text().await?;
    let map: HashMap<String, DbCharacter> = serde_json::from_str(&res_str)?;
    let characters = map.into_values().collect::<Vec<DbCharacter>>();
    let pathname = "/tmp/characters.json";
    let write_attempt = std::fs::write(pathname, serde_json::to_vec_pretty(&characters)?);

    let exist_status = Path::new(pathname).exists();
    let write_status = write_attempt.is_ok();
    debug!("exist_status: {}", exist_status);
    debug!("write_status: {}", write_status);

    Ok((exist_status, write_status))
}

#[cfg(test)]
mod tests {
    use super::{get_character_list, write_character_db};
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

    #[tokio::test]
    async fn write_db() {
        write_character_db().await.unwrap();
    }
}
