use self::upstream_avatar_config::AvatarConfig;
use crate::{
    handler::error::WorkerError,
    routes::{endpoint_types::List, honkai::traits::DbData},
};
use axum::{extract::Path, Json};
use reqwest::Method;
use std::{collections::HashMap, sync::Arc};
use tracing::info;

#[cfg(test)]
mod tests;
pub mod upstream_avatar_config;

/// Retrieves a single character info
pub async fn character(Path(character_id): Path<u32>) -> Result<Json<AvatarConfig>, WorkerError> {
    let now = std::time::Instant::now();

    let avatar_db: HashMap<String, AvatarConfig> = AvatarConfig::read().await?;

    let data = avatar_db
        .get(&character_id.to_string())
        .ok_or(WorkerError::EmptyBody)?;

    info!("Duration: {:?}", now.elapsed());
    Ok(Json(data.clone()))
}

pub async fn character_many(
    method: Method,
    character_ids: Option<Json<List<u32>>>,
) -> Result<Json<List<AvatarConfig>>, WorkerError> {
    let now = std::time::Instant::now();

    let avatar_db: HashMap<String, AvatarConfig> = AvatarConfig::read().await?;

    let ids = match (&method, character_ids) {
        (&Method::POST, Some(Json(List { list }))) => Some(list),
        _ => None,
    };

    let filtered: Arc<[AvatarConfig]> = avatar_db
        .iter()
        .filter(|(k, _)| ids.is_none() || ids.as_ref().unwrap().contains(&k.parse().unwrap()))
        .map(|(_, v)| v.clone())
        .collect();

    info!("Duration: {:?}", now.elapsed());
    Ok(Json(List::new(filtered.to_vec())))
}
