use self::categorizing::{DbCharacter, DbCharacterEidolon, DbCharacterSkillTree};
use crate::{
    handler::error::WorkerError,
    routes::{
        endpoint_types::List,
        honkai::{mhy_api::types_parsed::shared::DbAttributeProperty, traits::DbData},
    },
};
use anyhow::Result;
use axum::{extract::Path, Json};
use serde::de::DeserializeOwned;
use std::{collections::HashMap, fs, sync::Arc};
use tracing::{debug, info, instrument};

/// holds internal types for mhy's DB
// TODO: avoid conflicting type names with super::types
pub mod categorizing;
pub mod constants;
pub mod impls;

pub async fn all_characters() -> Result<Json<List<DbCharacter>>, WorkerError> {
    let now = std::time::Instant::now();

    let characters = DbCharacter::read().await?;
    let characters = characters
        .into_values()
        .map(|chara: DbCharacter| {
            // change MC name
            if chara.name == "{NICKNAME}" {
                let mut tb = chara.clone();
                tb.name = format!("Trailblazer ({})", chara.element);
                return tb;
            }
            chara
        })
        .collect();

    debug!("Duration {:?}", now.elapsed());

    Ok(Json(List::new(characters)))
}

#[instrument(ret, err)]
pub async fn character_by_id(Path(id): Path<u32>) -> Result<Json<DbCharacter>, WorkerError> {
    let now = std::time::Instant::now();

    let characters = DbCharacter::read().await?;
    let db_char = characters.get(&id.to_string()).cloned();

    debug!("Duration {:?}", now.elapsed());
    match db_char {
        Some(t) => Ok(Json(t)),
        None => Err(WorkerError::EmptyBody),
    }
}

#[instrument(ret, err)]
pub async fn trace_by_char_id(
    Path(id): Path<u32>,
) -> Result<Json<List<DbCharacterSkillTree>>, WorkerError> {
    let now = std::time::Instant::now();

    let db: HashMap<String, DbCharacterSkillTree> = DbCharacterSkillTree::read().await?;

    let traces: Arc<[DbCharacterSkillTree]> = db
        .iter()
        .filter(|(k, _)| k.starts_with(&id.to_string()))
        .map(|(_, v)| v.to_owned())
        .collect();

    info!("Duration: {:?}", now.elapsed());
    Ok(Json(traces.into()))
}
#[instrument(ret, err)]
pub async fn eidolon_by_char_id(
    Path(id): Path<u32>,
) -> Result<Json<List<DbCharacterEidolon>>, WorkerError> {
    let now = std::time::Instant::now();

    let db: HashMap<String, DbCharacterEidolon> = DbCharacterEidolon::read().await?;

    let eidolons: Arc<[DbCharacterEidolon]> = db
        .iter()
        .filter(|(k, _)| k.starts_with(&id.to_string()))
        .map(|(_, v)| v.to_owned())
        .collect();

    info!("Duration: {:?}", now.elapsed());
    Ok(Json(eidolons.into()))
}

pub async fn properties() -> Result<Json<List<DbAttributeProperty>>, WorkerError> {
    let now = std::time::Instant::now();
    let db: HashMap<String, DbAttributeProperty> = DbAttributeProperty::read().await?;
    let data: Arc<[DbAttributeProperty]> = db.into_values().collect();

    info!("Duration: {:?}", now.elapsed());
    Ok(Json(data.into()))
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
            let map: HashMap<String, T> = serde_json::from_str(&t)?;
            map.into_values().collect()
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
    use std::collections::HashMap;

    use crate::routes::honkai::{
        mhy_api::internal::{
            categorizing::{DbCharacter, DbCharacterSkill},
            get_db_list,
            impls::Queryable,
        },
        traits::DbData,
    };

    #[tokio::test]
    async fn calling() {
        let list: HashMap<String, DbCharacter> = DbCharacter::read().await.unwrap();
        let (_, kafka) = list.iter().find(|(_, e)| e.name.eq("Luocha")).unwrap();

        let skill_db = get_db_list::<DbCharacterSkill>("character_skills.json", "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_new/en/character_skills.json").await.unwrap();

        let skill_ids = kafka.skill_ids();
        let t = skill_db.find_many(skill_ids);

        let to_test = &t.iter().find(|e| e.id == 120304).unwrap();
        dbg!(&to_test.desc);
        dbg!(&to_test.params[0]);

        dbg!(&to_test.split_description());
    }
}
