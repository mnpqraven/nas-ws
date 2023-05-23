use super::types::GachaCfg;
use crate::handler::error::WorkerError;
use axum::Json;

pub async fn gacha_cfg() -> Result<Json<GachaCfg>, WorkerError> {
    Ok(Json(GachaCfg::default()))
}
