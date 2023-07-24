use self::{config::RelicConfig, set_config::RelicSetConfig};
use crate::{
    handler::error::WorkerError,
    routes::{endpoint_types::List, honkai::traits::DbData},
};
use axum::{extract::Path, Json};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use regex::Regex;

pub mod config;
pub mod set_config;

pub async fn relic_set() -> Result<Json<RelicSetConfig>, WorkerError> {
    let relic_set_db = RelicSetConfig::read().await?;
    todo!()
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
