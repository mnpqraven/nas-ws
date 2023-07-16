use crate::{
    handler::error::WorkerError,
    routes::{
        endpoint_types::List,
        honkai::{
            dm_api::equipment_config::{
                equipment_config::EquipmentConfig, equipment_skill_config::EquipmentSkillConfig,
            },
            traits::DbData,
        },
    },
};
use axum::{extract::Path, Json};
use reqwest::Method;
use std::{collections::HashMap, sync::Arc};
use tracing::info;

pub mod equipment_config;
pub mod equipment_skill_config;

pub async fn light_cone(Path(lc_id): Path<u32>) -> Result<Json<EquipmentConfig>, WorkerError> {
    let now = std::time::Instant::now();

    let db_metadata: HashMap<String, EquipmentConfig> = EquipmentConfig::read().await?;

    let res = db_metadata
        .get(&lc_id.to_string())
        .ok_or(WorkerError::EmptyBody)?;

    info!("Duration: {:?}", now.elapsed());
    Ok(Json(res.clone()))
}

pub async fn light_cone_many(
    method: Method,
    lc_ids: Option<Json<List<u32>>>,
) -> Result<Json<List<EquipmentConfig>>, WorkerError> {
    let now = std::time::Instant::now();
    let lc_ids = match (&method, lc_ids) {
        (&Method::POST, Some(Json(List { list }))) => Some(list),
        _ => None,
    };

    let db_metadata: HashMap<String, EquipmentConfig> = EquipmentConfig::read().await?;

    let res: Arc<[EquipmentConfig]> = db_metadata
        .iter()
        .filter(|(k, _)| lc_ids.is_none() || lc_ids.as_ref().unwrap().contains(&k.parse().unwrap()))
        .map(|(_, v)| v.clone())
        .collect();

    info!("Duration: {:?}", now.elapsed());
    Ok(Json(List::new(res.to_vec())))
}

pub async fn light_cone_skill(
    Path(lc_id): Path<u32>,
) -> Result<Json<EquipmentSkillConfig>, WorkerError> {
    let now = std::time::Instant::now();

    let db_metadata: HashMap<String, EquipmentSkillConfig> = EquipmentSkillConfig::read().await?;

    let res = db_metadata
        .get(&lc_id.to_string())
        .ok_or(WorkerError::EmptyBody)?;

    info!("Duration: {:?}", now.elapsed());
    Ok(Json(res.clone()))

}
pub async fn light_cone_skill_many(
    method: Method,
    lc_ids: Option<Json<List<u32>>>,
) -> Result<Json<List<EquipmentSkillConfig>>, WorkerError> {
    let now = std::time::Instant::now();
    let lc_ids = match (&method, lc_ids) {
        (&Method::POST, Some(Json(List { list }))) => Some(list),
        _ => None,
    };

    let db_metadata: HashMap<String, EquipmentSkillConfig> = EquipmentSkillConfig::read().await?;

    let res: Arc<[EquipmentSkillConfig]> = db_metadata
        .iter()
        .filter(|(k, _)| lc_ids.is_none() || lc_ids.as_ref().unwrap().contains(&k.parse().unwrap()))
        .map(|(_, v)| v.clone())
        .collect();

    info!("Duration: {:?}", now.elapsed());
    Ok(Json(List::new(res.to_vec())))
}
