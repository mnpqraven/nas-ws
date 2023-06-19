use super::{list_future_patch_banner, types::PatchBanner};
use crate::{handler::error::WorkerError, routes::endpoint_types::List};
use axum::Json;
use tracing::instrument;

#[instrument(ret)]
pub async fn handle() -> Result<Json<List<PatchBanner>>, WorkerError> {
    let Json(banners) = list_future_patch_banner().await?;
    Ok(Json(List::new(banners)))
}
