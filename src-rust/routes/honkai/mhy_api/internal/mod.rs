use self::{categorizing::DbCharacter, constants::CHARACTER_DICT};
use anyhow::Result;
use std::{collections::HashMap, fs, path::Path};
use tracing::debug;

/// holds internal types for mhy's DB
// TODO: avoid conflicting type names with super::types
pub mod categorizing;
pub mod constants;
#[cfg(test)]
mod runnables;

// NOTE: url fetching
pub async fn get_character_list() -> Result<Vec<DbCharacter>> {
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
    Ok(data)
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
