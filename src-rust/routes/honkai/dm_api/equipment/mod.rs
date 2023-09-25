use crate::{
    handler::error::WorkerError,
    routes::{
        endpoint_types::List,
        honkai::{
            dm_api::equipment::{
                equipment_config::EquipmentConfig,
                equipment_promotion_config::EquipmentPromotionConfig,
                equipment_skill_config::EquipmentSkillConfig,
            },
            traits::DbData,
        },
    },
};
use axum::{extract::Path, Json};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use regex::Regex;
use reqwest::Method;
use std::{collections::HashMap, sync::Arc};
use tracing::info;

pub mod equipment_config;
pub mod equipment_promotion_config;
pub mod equipment_skill_config;
pub mod stat_ranking;

pub async fn light_cone(Path(lc_id): Path<u32>) -> Result<Json<EquipmentConfig>, WorkerError> {
    let now = std::time::Instant::now();

    let db_metadata: HashMap<u32, EquipmentConfig> = EquipmentConfig::read().await?;

    let res = db_metadata.get(&lc_id).ok_or(WorkerError::EmptyBody)?;

    info!(
        "[/light_cone/{lc_id}/metadata] light_cone: {:?}",
        now.elapsed()
    );
    Ok(Json(res.clone()))
}
pub async fn light_cone_search(
    Path(lc_name): Path<String>,
) -> Result<Json<Option<EquipmentConfig>>, WorkerError> {
    // sanitizes params, only interested in chracters
    let regex = Regex::new("[^a-zA-Z0-9]").unwrap();
    let lc_name = regex.replace_all(&lc_name, "").to_string();

    let matcher = SkimMatcherV2::default();
    let db_metadata: HashMap<u32, EquipmentConfig> = EquipmentConfig::read().await?;
    let names: Vec<EquipmentConfig> = db_metadata
        .into_values()
        .filter(|v| matcher.fuzzy_match(&v.equipment_name, &lc_name).is_some())
        .collect();
    if names.is_empty() {
        return Ok(Json(None));
    }

    Ok(Json(names.get(0).cloned()))
}

pub async fn light_cones(
    method: Method,
    lc_ids: Option<Json<List<u32>>>,
) -> Result<Json<List<EquipmentConfig>>, WorkerError> {
    let now = std::time::Instant::now();
    let lc_ids = match (&method, lc_ids) {
        (&Method::POST, Some(Json(List { list }))) => Some(list),
        _ => None,
    };

    let db_metadata = EquipmentConfig::read().await?;

    let res: Arc<[EquipmentConfig]> = db_metadata
        .iter()
        .filter(|(k, _)| lc_ids.is_none() || lc_ids.as_ref().unwrap().contains(k))
        .map(|(_, v)| v.clone())
        .collect();

    info!(
        "[/light_cone/metadata] light_cone_many: {:?}",
        now.elapsed()
    );
    Ok(Json(List::new(res.to_vec())))
}

pub async fn lc_skill(Path(lc_id): Path<u32>) -> Result<Json<EquipmentSkillConfig>, WorkerError> {
    let now = std::time::Instant::now();

    let db_metadata = EquipmentSkillConfig::read().await?;

    let res = db_metadata.get(&lc_id).ok_or(WorkerError::EmptyBody)?;

    info!(
        "[/light_cone/{lc_id}/skill] light_cone_skill: {:?}",
        now.elapsed()
    );
    Ok(Json(res.clone()))
}

pub async fn lc_skills(
    method: Method,
    lc_ids: Option<Json<List<u32>>>,
) -> Result<Json<List<EquipmentSkillConfig>>, WorkerError> {
    let now = std::time::Instant::now();
    let lc_ids = match (&method, lc_ids) {
        (&Method::POST, Some(Json(List { list }))) => Some(list),
        _ => None,
    };

    // TODO: split reader for perf
    let db_metadata = EquipmentSkillConfig::read().await?;

    let res: Arc<[EquipmentSkillConfig]> = db_metadata
        .iter()
        .filter(|(k, _)| lc_ids.is_none() || lc_ids.as_ref().unwrap().contains(k))
        .map(|(_, v)| v.clone())
        .collect();

    info!(
        "[/light_cone/skill] light_cone_skill_many: {:?}",
        now.elapsed()
    );
    Ok(Json(List::new(res.to_vec())))
}

pub async fn lc_promotion(
    Path(lc_id): Path<u32>,
) -> Result<Json<EquipmentPromotionConfig>, WorkerError> {
    let now = std::time::Instant::now();

    let promotion_db = EquipmentPromotionConfig::read().await?;

    let res = promotion_db.get(&lc_id).ok_or(WorkerError::EmptyBody)?;

    info!("Duration: {:?}", now.elapsed());
    Ok(Json(res.clone()))
}

pub async fn lc_promotions(
    method: Method,
    lc_ids: Option<Json<List<u32>>>,
) -> Result<Json<List<EquipmentPromotionConfig>>, WorkerError> {
    let now = std::time::Instant::now();
    let lc_ids = match (&method, lc_ids) {
        (&Method::POST, Some(Json(List { list }))) => Some(list),
        _ => None,
    };

    let db_metadata = EquipmentPromotionConfig::read().await?;

    let res: Arc<[EquipmentPromotionConfig]> = db_metadata
        .iter()
        .filter(|(k, _)| lc_ids.is_none() || lc_ids.as_ref().unwrap().contains(k))
        .map(|(_, v)| v.clone())
        .collect();

    info!(
        "[/light_cone/promotion] light_cone_promotion_many: {:?}",
        now.elapsed()
    );
    Ok(Json(List::new(res.to_vec())))
}
