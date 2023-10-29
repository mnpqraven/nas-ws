use self::{
    eidolon::AvatarRankConfig, promotion_config::AvatarPromotionConfig,
    types::AvatarConfig,
};
use crate::{
    handler::error::WorkerError,
    routes::{endpoint_types::List, honkai::traits::DbData},
};
use axum::{extract::Path, Json};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use regex::Regex;
use reqwest::Method;
use std::{collections::HashMap, sync::Arc};
use tracing::info;

pub mod eidolon;
pub mod promotion_config;
#[cfg(test)]
mod tests;
pub mod types;
pub mod rpc;

/// Retrieves a single character info
pub async fn character(Path(character_id): Path<u32>) -> Result<Json<AvatarConfig>, WorkerError> {
    let now = std::time::Instant::now();

    let avatar_db: HashMap<u32, AvatarConfig> = AvatarConfig::read().await?;

    let data = avatar_db.get(&character_id).ok_or(WorkerError::EmptyBody)?;

    info!("Duration: {:?}", now.elapsed());
    Ok(Json(data.clone()))
}

pub async fn character_by_name(
    Path(character_name): Path<String>,
) -> Result<Json<Option<AvatarConfig>>, WorkerError> {
    let regex = Regex::new("[^a-zA-Z0-9]").unwrap();
    let character_name = regex.replace_all(&character_name, "").to_string();
    let matcher = SkimMatcherV2::default();

    let avatar_db: HashMap<u32, AvatarConfig> = AvatarConfig::read().await?;

    let data: Vec<AvatarConfig> = avatar_db
        .into_values()
        .filter(|v| {
            let fuzz_result = matcher.fuzzy_match(&v.avatar_name, &character_name);
            fuzz_result.is_some()
        })
        .collect();
    if data.is_empty() {
        return Ok(Json(None));
    }

    Ok(Json(data.get(0).cloned()))
}

pub async fn character_many(
    method: Method,
    character_ids: Option<Json<List<u32>>>,
) -> Result<Json<List<AvatarConfig>>, WorkerError> {
    let now = std::time::Instant::now();

    let avatar_db: HashMap<u32, AvatarConfig> = AvatarConfig::read().await?;

    let ids = match (&method, character_ids) {
        (&Method::POST, Some(Json(List { list }))) => Some(list),
        _ => None,
    };

    let filtered: Arc<[AvatarConfig]> = avatar_db
        .iter()
        .filter(|(k, v)| (ids.is_none() || ids.as_ref().unwrap().contains(k)) && v.release)
        .map(|(_, v)| v.clone())
        .collect();

    info!("Duration: {:?}", now.elapsed());
    Ok(Json(List::new(filtered.to_vec())))
}

pub async fn promotion(
    Path(character_id): Path<u32>,
) -> Result<Json<AvatarPromotionConfig>, WorkerError> {
    let now = std::time::Instant::now();
    let eidolon_db: HashMap<u32, AvatarPromotionConfig> = AvatarPromotionConfig::read().await?;

    let data = eidolon_db
        .get(&character_id)
        .ok_or(WorkerError::NotFound(character_id.to_string()))?;

    info!("Duration: {:?}", now.elapsed());
    Ok(Json(data.clone()))
}

pub async fn eidolon(
    Path(character_id): Path<u32>,
) -> Result<Json<List<AvatarRankConfig>>, WorkerError> {
    let character = AvatarConfig::read().await?;
    let eidolon_db = AvatarRankConfig::read().await?;
    let ranks = &character
        .get(&character_id)
        .ok_or(WorkerError::NotFound(character_id.to_string()))?
        .rank_idlist;
    let eidolons = ranks
        .iter()
        .map(|rank_id| {
            eidolon_db
                .get(rank_id)
                .ok_or(WorkerError::NotFound(rank_id.to_string()))
                .cloned()
                .unwrap()
        })
        .collect();
    Ok(Json(List::new(eidolons)))
}
