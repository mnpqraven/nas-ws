use std::sync::Arc;

use self::{
    config::RelicConfig, set_config::RelicSetConfig, set_skill_config::RelicSetSkillConfig,
};
use crate::{
    handler::error::WorkerError,
    routes::{endpoint_types::List, honkai::traits::DbData},
};
use axum::{extract::Path, Json};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use regex::Regex;
use reqwest::Method;

pub mod config;
pub mod set_config;
pub mod set_skill_config;

pub async fn relic_set(Path(set_id): Path<u32>) -> Result<Json<RelicSetConfig>, WorkerError> {
    let relic_set_db = RelicSetConfig::read().await?;
    let data = relic_set_db
        .get(&set_id)
        .ok_or(WorkerError::NotFound(set_id.to_string()))?;

    Ok(Json(data.clone()))
}

pub async fn relic_set_many() -> Result<Json<List<RelicSetConfig>>, WorkerError> {
    let relic_set_db = RelicSetConfig::read().await?;
    todo!()
}

pub async fn relic_set_search(
    Path(name): Path<String>,
) -> Result<Json<Option<RelicSetConfig>>, WorkerError> {
    // sanitizes params, only interested in chracters
    let regex = Regex::new("[^a-zA-Z0-9]").unwrap();
    let matcher = SkimMatcherV2::default();

    let relic_set_db = RelicSetConfig::read().await?;

    let relic_name = regex.replace_all(&name, "").to_string();
    let data: Vec<RelicSetConfig> = relic_set_db
        .into_values()
        .filter(|v| {
            let fuzz_result = matcher.fuzzy_match(&v.set_name, &relic_name);
            fuzz_result.is_some()
        })
        .collect();
    if data.is_empty() {
        return Ok(Json(None));
    }
    Ok(Json(data.get(0).cloned()))
}

pub async fn relics_by_set(
    Path(set_id): Path<u32>,
) -> Result<Json<List<RelicConfig>>, WorkerError> {
    todo!()
}

pub async fn set_bonus(Path(set_id): Path<u32>) -> Result<Json<RelicSetSkillConfig>, WorkerError> {
    let bonus_db = RelicSetSkillConfig::read().await?;
    let data = bonus_db
        .get(&set_id)
        .cloned()
        .ok_or(WorkerError::NotFound(set_id.to_string()))?;
    Ok(Json(data))
}

pub async fn set_bonus_many(
    method: Method,
    relic_ids: Option<Json<List<u32>>>,
) -> Result<Json<List<RelicSetSkillConfig>>, WorkerError> {
    let bonus_db = RelicSetSkillConfig::read().await?;
    let ids = match (&method, relic_ids) {
        (&Method::POST, Some(Json(List { list }))) => Some(list),
        _ => None,
    };

    let data: Arc<[RelicSetSkillConfig]> = bonus_db
        .iter()
        .filter(|(k, v)| ids.is_none() || ids.as_ref().unwrap().contains(k))
        .map(|(_, v)| v.clone())
        .collect();
    Ok(Json(List::new(data.to_vec())))
}
