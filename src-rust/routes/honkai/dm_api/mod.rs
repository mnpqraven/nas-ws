use std::collections::HashMap;

use serde::Serialize;
use tracing::{debug, info};

use crate::{handler::error::WorkerError, routes::honkai::dm_api::types::SkillTreeConfig};

use self::types::SkillTreeConfigWrapper;

use super::mhy_api::internal::{categorizing::DbCharacterSkillTree, impls::DbData};

pub mod types;

#[derive(Debug, Serialize)]
pub struct BigTraceInfo {
    pub id: u32,
    pub name: String,
    pub desc: String,
    pub params: Vec<f32>,
}

const TEXT_MAP: &str =
    "https://raw.githubusercontent.com/Dimbreath/StarRailData/master/TextMap/TextMapEN.json";
const DM_TRACE_DB: &str = "https://raw.githubusercontent.com/Dimbreath/StarRailData/master/ExcelOutput/AvatarSkillTreeConfig.json";

async fn write_big_trace() -> Result<(), WorkerError> {
    let mut big_trace_map: HashMap<String, BigTraceInfo> = HashMap::new();

    let desc_chunk = reqwest::get(TEXT_MAP).await?.text().await?;
    let desc_chunk: HashMap<String, String> = serde_json::from_str(&desc_chunk)?;

    let dm_trace_db = reqwest::get(DM_TRACE_DB).await?.text().await?;
    let dm_trace_db: HashMap<String, HashMap<String, SkillTreeConfig>> =
        serde_json::from_str(&dm_trace_db)?;

    for (k, inner_map) in dm_trace_db.into_iter() {
        // only big traces contains `_skilltree` in icon
        if let Some(config) = inner_map.get(&"1".to_string()) {
            // is big trace
            if config.icon_path.contains("_SkillTree") {
                let mut name = String::new();
                let mut desc = String::new();
                if !config.point_name.is_empty() {
                    let hash = config.point_name.clone();
                    let hashed = get_stable_hash(&hash);

                    if let Some(value) = desc_chunk.get(&hashed.to_string()) {
                        name = value.to_string();
                    }
                }
                if !config.point_desc.is_empty() {
                    let hash = config.point_desc.clone();
                    let hashed = get_stable_hash(&hash);
                    if let Some(value) = desc_chunk.get(&hashed.to_string()) {
                        desc = format_desc(value);
                    }
                }

                let trace_info = BigTraceInfo {
                    id: k.parse::<u32>()?,
                    name,
                    desc,
                    params: config.param_list.iter().map(|e| e.value).collect(),
                };
                println!("{:?}", trace_info);
                big_trace_map.insert(k, trace_info);
            }
        }
    }

    // TODO: writer
    // TODO: trait implementation
    info!("{:?}", big_trace_map);
    std::fs::write(
        "/tmp/big_traces.json",
        serde_json::to_string_pretty(&big_trace_map)?,
    )?;

    Ok(())
}

fn format_desc(desc: &str) -> String {
    desc.replace("<unbreak>", "").replace("</unbreak>", "")
}

fn get_stable_hash(hash: &str) -> i32 {
    let mut hash1: i32 = 5381;
    let mut hash2: i32 = hash1;

    let mut i = 0;
    while i < hash.len() && hash.as_bytes()[i] as char != '\0' {
        hash1 = ((hash1 << 5).wrapping_add(hash1)) ^ hash.as_bytes()[i] as i32;
        if i == hash.len() - 1 || hash.as_bytes()[i + 1] as char == '\0' {
            break;
        }
        hash2 = ((hash2 << 5).wrapping_add(hash2)) ^ hash.as_bytes()[i + 1] as i32;
        i += 2;
    }

    hash1.wrapping_add(hash2.wrapping_mul(1566083941))
}

#[cfg(test)]
mod tests {
    use super::{get_stable_hash, write_big_trace};

    #[test]
    fn hasher() {
        let hashed = get_stable_hash("SkillPointDesc_1102101");
        assert_eq!(hashed, 944602705)
    }

    #[tokio::test]
    async fn write() {
        write_big_trace().await.unwrap();
    }
}
