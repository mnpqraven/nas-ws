use self::upstream_avatar_config::{AvatarConfig, UpstreamAvatarConfig};
use crate::{
    builder::AsyncInto,
    handler::error::WorkerError,
    routes::{
        endpoint_types::List,
        honkai::{dm_api::types::TextMap, mhy_api::internal::impls::DbData},
    },
};
use axum::{extract::Path, Json};
use reqwest::Method;
use std::{collections::HashMap, sync::Arc};
use tracing::info;

#[cfg(test)]
mod tests;
pub mod upstream_avatar_config;

pub async fn character(
    method: Method,
    character_id: Option<Path<u32>>,
    character_ids: Option<Json<List<u32>>>,
) -> Result<Json<List<AvatarConfig>>, WorkerError> {
    let now = std::time::Instant::now();

    let avatar_db: HashMap<String, UpstreamAvatarConfig> = UpstreamAvatarConfig::read().await?;

    let ids = match (&method, character_id, character_ids) {
        (&Method::GET, Some(Path(id)), _) => Some(vec![id]),
        (
            &Method::POST,
            _,
            Some(Json(List {
                list: character_ids,
            })),
        ) => Some(character_ids),
        _ => None,
    };

    let filtered_upstream: Arc<[UpstreamAvatarConfig]> = avatar_db
        .iter()
        .filter(|(k, _)| ids.is_none() || ids.as_ref().unwrap().contains(&k.parse().unwrap()))
        .map(|(_, v)| v.clone())
        .collect();

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
