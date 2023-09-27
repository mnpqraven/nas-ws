use super::honkai::{
    dm_api::atlas::rpc::dm_atlas_route, jade_estimate::rpc::jadeestimate_route,
    probability_rate::rpc::probabilityrate_route,
};
use axum::Router;

pub fn rpc_routes() -> Router {
    Router::new()
        .route("/dm.atlas.SignatureAtlasService/*rpc", dm_atlas_route())
        .route(
            "/jadeestimate.JadeEstimateService/*rpc",
            jadeestimate_route(),
        )
        .route(
            "/probabilityrate.ProbabilityRateService/*rpc",
            probabilityrate_route(),
        )
}
