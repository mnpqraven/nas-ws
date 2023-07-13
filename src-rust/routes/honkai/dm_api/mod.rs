use self::equipment_config::{
    equipment_config::EquipmentConfig, equipment_skill_config::EquipmentSkillConfig,
};
use crate::{
    handler::{error::WorkerError, FromAxumResponse},
    routes::{
        endpoint_types::List,
        honkai::{
            dm_api::{
                desc_param::{get_sorted_params, ParameterizedDescription},
                hash::TextHash,
                skill_tree_config::skill_tree_config::SkillTreeConfig,
                types::TextMap,
            },
            mhy_api::internal::{categorizing::SkillType::BPSkill, impls::DbData},
            patch::types::SimpleSkill,
        },
    },
};
use axum::{extract::Path, Json};
use response_derive::JsonResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::BufReader, sync::Arc};
use tracing::info;
use vercel_runtime::{Body, Response, StatusCode};

pub mod atlas;
pub mod avatar_config;
mod constants;
pub mod desc_param;
pub mod equipment_config;
pub mod hash;
pub mod impls;
pub mod skill_tree_config;
pub mod types;

#[derive(Debug, Serialize, Deserialize, Clone, JsonResponse, JsonSchema)]
pub struct BigTraceInfo {
    pub id: u32,
    pub name: String,
    pub desc: String,
    pub params: Vec<f64>,
}

const DM_TRACE_DB: &str = "https://raw.githubusercontent.com/Dimbreath/StarRailData/master/ExcelOutput/AvatarSkillTreeConfig.json";

#[cfg(target_os = "windows")]
pub const BIG_TRACE_LOCAL: &str = "c:\\tmp\\big_traces.json";
#[cfg(target_os = "linux")]
pub const BIG_TRACE_LOCAL: &str = "/tmp/big_traces.json";

#[derive(Debug, Serialize, Deserialize, JsonResponse, JsonSchema)]
pub struct LightCone {
    pub metadata: EquipmentConfig,
    pub skill: EquipmentSkillConfig,
}

pub async fn read_by_char_id(
    Path(char_id): Path<u32>,
) -> Result<Json<List<SimpleSkill>>, WorkerError> {
    let now = std::time::Instant::now();

    if !std::path::Path::new(BIG_TRACE_LOCAL).try_exists()? {
        write_big_trace().await?;
    }
    let file = std::fs::File::open(BIG_TRACE_LOCAL)?;
    let reader = BufReader::new(file);
    let db: HashMap<String, BigTraceInfo> = serde_json::from_reader(reader)?;

    let big_traces: Arc<[SimpleSkill]> = db
        .iter()
        .filter(|(k, _)| k.starts_with(&char_id.to_string()))
        .map(|(_, v)| {
            let description: ParameterizedDescription = v.desc.clone().into();
            let params = get_sorted_params(v.params.clone(), &v.desc)
                .iter()
                .map(|e| e.to_string())
                .collect();

            SimpleSkill {
                id: v.id,
                name: v.name.clone(),
                ttype: BPSkill,
                description: description.0,
                params: vec![params],
            }
        })
        .collect();

    info!("Duration: {:?}", now.elapsed());

    Ok(Json(big_traces.into()))
}

pub async fn write_big_trace() -> Result<(), WorkerError> {
    let mut big_trace_map: HashMap<String, BigTraceInfo> = HashMap::new();

    let text_map: HashMap<String, String> = TextMap::read().await?;

    let dm_trace_db = reqwest::get(DM_TRACE_DB).await?.text().await?;
    // depth 2 reads
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
                    let hashed = TextHash::get_stable_hash(&hash);

                    if let Some(value) = text_map.get(&hashed.to_string()) {
                        name = value.to_string();
                    }
                }
                if !config.point_desc.is_empty() {
                    let hash = config.point_desc.clone();
                    let hashed = TextHash::get_stable_hash(&hash);
                    if let Some(value) = text_map.get(&hashed.to_string()) {
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
        BIG_TRACE_LOCAL,
        serde_json::to_string_pretty(&big_trace_map)?,
    )?;

    Ok(())
}

fn format_desc(desc: &str) -> String {
    desc.replace("<unbreak>", "").replace("</unbreak>", "")
}

#[cfg(test)]
mod tests {
    use crate::routes::{
        endpoint_types::List,
        honkai::dm_api::{equipment_config::light_cone, hash::TextHash},
    };
    use axum::Json;
    use reqwest::Method;

    #[test]
    fn hasher() {
        let hashed = TextHash::get_stable_hash("SkillPointDesc_1102101");
        assert_eq!(hashed, 944602705)
    }

    #[tokio::test]
    async fn eq() {
        let _left = light_cone(Method::POST, None, Some(Json(List::new(vec![23005]))))
            .await
            .unwrap();
        dbg!(&_left);
    }
}
