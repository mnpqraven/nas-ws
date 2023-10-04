use self::types::Item;
use crate::{
    handler::error::WorkerError,
    routes::{endpoint_types::List, honkai::traits::DbData},
};
use axum::Json;

pub mod types;

pub async fn item_list() -> Result<Json<List<Item>>, WorkerError> {
    let db = Item::read().await?;

    let list = db.into_values().collect();

    Ok(Json(List::new(list)))
}
