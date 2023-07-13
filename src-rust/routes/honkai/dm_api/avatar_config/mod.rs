use self::upstream_avatar_config::{AvatarConfig, UpstreamAvatarConfig};
use crate::{
    builder::AsyncInto,
    handler::error::WorkerError,
    routes::{
        endpoint_types::List,
        honkai::{dm_api::types::TextMap, mhy_api::internal::impls::DbData},
    },
};
use axum::Json;
use std::{collections::HashMap, sync::Arc};
use tracing::info;

#[cfg(test)]
mod tests;
mod upstream_avatar_config;

pub async fn avatar_list(
    character_ids: Option<Json<List<u32>>>,
) -> Result<Json<List<AvatarConfig>>, WorkerError> {
    let now = std::time::Instant::now();

    let avatar_db: HashMap<String, UpstreamAvatarConfig> = UpstreamAvatarConfig::read().await?;

    let filtered_upstream: Arc<[UpstreamAvatarConfig]> = match character_ids {
        Some(Json(List {
            list: character_ids,
        })) => avatar_db
            .into_iter()
            .filter(|(k, _)| character_ids.contains(&k.parse::<u32>().unwrap()))
            .map(|(_, v)| v)
            .collect(),
        None => avatar_db.into_values().collect(),
    };

    let text_map: HashMap<String, String> = TextMap::read().await?;
    // CRITICAL
    // WARN: massive time sink (> 10s !!!)
    let data: Result<Vec<AvatarConfig>, WorkerError> = filtered_upstream
        .iter()
        .map(|v| {
            let res = v.clone().into_using_resource(&text_map)?;
            Ok(res)
        })
        .collect::<Vec<Result<AvatarConfig, WorkerError>>>()
        .into_iter()
        .collect();

    info!("Duration: {:?}", now.elapsed());
    Ok(Json(List::new(data?)))
}
