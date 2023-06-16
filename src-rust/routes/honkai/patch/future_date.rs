use super::{list_future_patch_date, types::Patch};
use crate::{handler::error::WorkerError, routes::endpoint_types::List};
use axum::Json;

pub async fn handle() -> Result<Json<List<Patch>>, WorkerError> {
    let Json(patches) = list_future_patch_date().await?;
    Ok(Json(List::new(patches)))
}
