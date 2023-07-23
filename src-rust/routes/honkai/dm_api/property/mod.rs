use self::config::AvatarPropertyConfig;
use crate::{
    handler::error::WorkerError,
    routes::{endpoint_types::List, honkai::traits::DbData},
};
use axum::Json;

pub mod config;

pub async fn property() -> Result<Json<List<AvatarPropertyConfig>>, WorkerError> {
    let property_db = AvatarPropertyConfig::read().await?;

    Ok(Json(List::new(property_db.into_values().collect())))
}
