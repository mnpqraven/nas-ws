use super::honkai::{
    dm_api::atlas::{
        rpc::atlas::signature_atlas_service_server::SignatureAtlasServiceServer, SignatureAtlas,
    },
    jade_estimate::{
        rpc::jadeestimate::jade_estimate_service_server::JadeEstimateServiceServer,
        types::JadeEstimateResponse,
    },
    probability_rate::{
        rpc::probabilityrate::probability_rate_service_server::ProbabilityRateServiceServer,
        types::ProbabilityRateResponse,
    },
};
use axum::{routing::any_service, Router};
use tonic_web::enable;

pub fn rpc_routes() -> Router {
    let atlas_sv = any_service(enable(SignatureAtlasServiceServer::new(
        SignatureAtlas::default(),
    )));
    let jadeestimate_sv = any_service(enable(JadeEstimateServiceServer::new(
        JadeEstimateResponse::default(),
    )));
    let probabilityrate_sv = any_service(enable(ProbabilityRateServiceServer::new(
        ProbabilityRateResponse::default(),
    )));

    Router::new()
        .route("/dm.atlas.SignatureAtlasService/*rpc", atlas_sv)
        .route("/jadeestimate.JadeEstimateService/*rpc", jadeestimate_sv)
        .route(
            "/probabilityrate.ProbabilityRateService/*rpc",
            probabilityrate_sv,
        )
}
