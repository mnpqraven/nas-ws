use super::LightCone;
use crate::{
    handler::error::WorkerError,
    routes::{
        endpoint_types::List,
        honkai::{
            dm_api::equipment_config::{
                equipment_config::EquipmentConfig, equipment_skill_config::EquipmentSkillConfig,
            },
            mhy_api::internal::impls::DbData,
        },
    },
};
use axum::{extract::Path, Json};
use reqwest::Method;
use std::{collections::HashMap, sync::Arc};
use tracing::info;

pub mod equipment_config;
pub mod equipment_skill_config;

pub async fn light_cone(
    method: Method,
    lc_id: Option<Path<u32>>,
    lc_ids: Option<Json<List<u32>>>,
) -> Result<Json<List<LightCone>>, WorkerError> {
    let now = std::time::Instant::now();

    let db_skill: HashMap<String, EquipmentSkillConfig> = EquipmentSkillConfig::read().await?;

    let db_metadata: HashMap<String, EquipmentConfig> = EquipmentConfig::read().await?;
    let db_metadata_arced = Arc::new(db_metadata);

    let res = db_metadata_arced
        .keys()
        .filter(|key| match (&method, &lc_id, &lc_ids) {
            (&Method::GET, Some(Path(id)), _) => id.eq(&key.parse::<u32>().unwrap()),
            (&Method::POST, _, Some(Json(List { list: ids }))) => {
                ids.contains(&key.parse::<u32>().unwrap())
            }
            _ => true,
        })
        .map(|lc_id| {
            let metadata = db_metadata_arced.get(&lc_id.to_string()).cloned().unwrap();
            let skill = db_skill.get(&lc_id.to_string()).cloned().unwrap();
            LightCone { metadata, skill }
        })
        .collect();

    info!("Duration: {:?}", now.elapsed());
    Ok(Json(List::new(res)))
}
