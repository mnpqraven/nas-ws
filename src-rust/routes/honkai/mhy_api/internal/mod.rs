use self::{categorizing::Character, constants::CHARACTER_DICT};
use std::{collections::HashMap, error::Error};
use tracing::debug;

/// holds internal types for mhy's DB
// TODO: avoid conflicting type names with super::types
pub mod categorizing;
mod constants;
#[cfg(test)]
mod runnables;

// NOTE: url fetching
pub async fn get_character_list() -> Result<Vec<Character>, Box<dyn Error>> {
    debug!("fetching from fallback url");
    let res_str: String = reqwest::get(CHARACTER_DICT).await?.text().await?;
    let map: HashMap<String, Character> = serde_json::from_str(&res_str)?;
    Ok(map.into_values().collect::<Vec<Character>>())
}
