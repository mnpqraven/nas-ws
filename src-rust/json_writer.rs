use std::{collections::HashMap, path::Path};

use anyhow::Result;
use nas_ws::routes::honkai::mhy_api::internal::categorizing::{DbCharacter, DbCharacterSkill};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

const URL: &str = "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/";
const DICT_PATH: &str = "index_new/en/";
const CHARACTERS: &str = "characters.json";
const CHARACTER_SKILLS: &str = "character_skills.json";

#[cfg(target_os = "windows")]
const TARGET_PATH: &str = "../dump_data";
#[cfg(target_os = "linux")]
const TARGET_PATH: &str = "/tmp/";

#[tokio::main]
async fn main() -> Result<()> {
    write_data::<DbCharacter>(CHARACTERS).await?;
    write_data::<DbCharacterSkill>(CHARACTER_SKILLS).await?;

    Ok(())
}

async fn hashed<T: DeserializeOwned>(url: Url) -> Result<Vec<T>> {
    let data = reqwest::get(url.as_str()).await?.text().await?;
    let map: HashMap<String, T> = serde_json::from_str(&data)?;
    Ok(map.into_values().collect::<Vec<T>>())
}

async fn write_data<T>(filename: &str) -> Result<()>
where
    T: Serialize + DeserializeOwned,
{
    let url = Url::parse(URL)?.join(DICT_PATH)?.join(filename)?;
    println!("{:?}", url.as_str());
    let data: Vec<T> = hashed(url).await?;
    #[cfg(target_os = "windows")]
    {
        create_dir_all(TARGET_PATH)?
    }

    let path = Path::new(TARGET_PATH).join(filename);
    let data_vec = serde_json::to_vec_pretty(&data)?;
    std::fs::write(path, data_vec)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use url::Url;

    #[test]
    fn url() {
        let url: &str = "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/";
        let dict_path: &str = "index_new/en/";
        let characters: &str = "characters.json";

        let to_debug = Url::parse(url)
            .unwrap()
            .join(dict_path)
            .unwrap()
            .join(characters)
            .unwrap();
        println!("{:?}", to_debug);
        // let parsed = Url::parse(to_debug).unwrap().join(characters).unwrap();
        assert_eq!(to_debug.as_str(), String::from("https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_new/en/characters.json"))
    }
}
