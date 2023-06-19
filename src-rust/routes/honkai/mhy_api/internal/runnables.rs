use crate::{
    handler::error::WorkerError,
    routes::honkai::mhy_api::internal::constants::{
        CHARACTER_REMOTE, RELIC_PIECES_DICT, RELIC_SET_DICT,
    },
};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::categorizing::{DbCharacter, RelicInfo, RelicSet};
use std::{collections::HashMap, error::Error};

#[async_trait]
pub trait HasPath: Send + Sync {
    type T: Serialize + DeserializeOwned;
    fn path_data() -> (&'static str, &'static str);
    async fn try_write_disk(local_path: &str) -> Result<String, WorkerError> {
        let (_, fallback_url) = Self::path_data();
        let data = reqwest::get(fallback_url).await?.text().await?;
        std::fs::write(local_path, data.clone())?;
        Ok(data)
    }
}
#[async_trait]
pub trait DbData {
    type TValue: Serialize + DeserializeOwned + HasPath;
    /// read the local file for data, lazily writes from fallback url if not
    /// exist
    /// return hashmap with the db struct's PK as keys
    async fn read() -> Result<HashMap<String, Self::TValue>, WorkerError> {
        let (local_path, _) = Self::TValue::path_data();
        let str_data: String = match std::path::Path::new(local_path).exists() {
            true => std::fs::read_to_string(local_path)?,
            // lazily writes data
            false => Self::TValue::try_write_disk(local_path).await?,
        };
        Ok(serde_json::from_str(&str_data)?)
    }
}

/*
#[tokio::test]
async fn get_relic_sets() -> Result<(), Box<dyn Error>> {
    let res_str = reqwest::get(RELIC_SET_DICT).await?.text().await?;

    let _map: HashMap<String, RelicSet> = serde_json::from_str(&res_str)?;

    // println!("{:?}", map);

    Ok(())
}

#[tokio::test]
async fn get_relic_pieces() -> Result<(), Box<dyn Error>> {
    let res_str = reqwest::get(RELIC_PIECES_DICT).await?.text().await?;

    let _map: HashMap<String, RelicInfo> = serde_json::from_str(&res_str)?;

    // println!("{:?}", map);

    Ok(())
}

#[tokio::test]
async fn get_character_list() -> Result<(), Box<dyn Error>> {
    let res_str = reqwest::get(CHARACTER_REMOTE).await?.text().await?;

    let map: HashMap<String, DbCharacter> = serde_json::from_str(&res_str)?;
    let characters: Vec<DbCharacter> = map.into_values().collect();
    for character in characters.iter() {
        println!("{}", character.name);
    }

    Ok(())
}
*/
