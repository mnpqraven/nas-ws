use self::{categorizing::Character, constants::CHARACTER_DICT};
use std::{collections::HashMap, error::Error, fs};
use tracing::debug;

/// holds internal types for mhy's DB
// TODO: avoid conflicting type names with super::types
pub mod categorizing;
mod constants;
#[cfg(test)]
mod runnables;

// NOTE: url fetching
pub async fn get_character_list() -> Result<Vec<Character>, Box<dyn Error>> {
    // let res_str: String = reqwest::get(CHARACTER_DICT).await?.text().await?;
    // let map: HashMap<String, Character> = serde_json::from_str(&res_str)?;

    let data = fs::read_to_string("/tmp/characters.json")?;
    let t: Vec<Character> = serde_json::from_str(&data)?;
    Ok(t)
}
