use self::{categorizing::Character, constants::CHARACTER_DICT};
use std::{collections::HashMap, env, error::Error, fs, path::Path};
use tracing::debug;

/// holds internal types for mhy's DB
// TODO: avoid conflicting type names with super::types
mod categorizing;
mod constants;
#[cfg(test)]
mod runnables;

// TODO: set url fetching as fallback, default to dir lookup
async fn get_character_list() -> Result<Vec<Character>, Box<dyn Error>> {
    let dir = env::current_dir()?;
    let res_str: String = match Path::new(&dir).join("characters.json").exists() {
        true => {
            debug!("using cached asset");
            fs::read_to_string("characters.json")?
        }
        false => {
            debug!("using fallback url");
            reqwest::get(CHARACTER_DICT).await?.text().await?
        }
    };
    let map: HashMap<String, Character> = serde_json::from_str(&res_str)?;
    Ok(map.into_values().collect::<Vec<Character>>())
}

/// NOTE: /api/cron/characters_db
pub async fn write_character_list() -> Result<(), Box<dyn Error>> {
    debug!("write_character_list");
    let characters = get_character_list().await?;
    let dir = env::current_dir()?;
    fs::write(
        Path::new(&dir).join("characters.json"),
        serde_json::to_vec_pretty(&characters)?,
    )?;
    debug!("write_character_list completed");
    Ok(())
}
