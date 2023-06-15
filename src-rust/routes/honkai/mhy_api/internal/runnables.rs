use crate::routes::honkai::mhy_api::internal::constants::{
    CHARACTER_DICT, RELIC_PIECES_DICT, RELIC_SET_DICT,
};

use super::categorizing::{Character, RelicInfo, RelicSet};
use std::{collections::HashMap, error::Error};

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
    let res_str = reqwest::get(CHARACTER_DICT).await?.text().await?;

    let map: HashMap<String, Character> = serde_json::from_str(&res_str)?;
    let characters: Vec<Character> = map.into_values().collect();
    for character in characters.iter() {
        println!("{}", character.name);
    }

    Ok(())
}
