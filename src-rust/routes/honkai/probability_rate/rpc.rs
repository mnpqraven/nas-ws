use self::probabilityrate::probability_rate_service_server::*;
use super::types::{ProbabilityRatePayload, ProbabilityRateResponse, ReducedSim};
use crate::routes::honkai::probability_rate::handle;
use axum::{
    routing::{any_service, MethodRouter},
    Json,
};
use tonic::{Request, Response, Status};
use tonic_web::enable;

pub mod probabilityrate {
    tonic::include_proto!("probabilityrate");
}

#[tonic::async_trait]
impl ProbabilityRateService for ProbabilityRateResponse {
    async fn post(
        &self,
        req: Request<probabilityrate::ProbabilityRatePayload>,
    ) -> Result<Response<probabilityrate::ProbabilityRateResponse>, Status> {
        let req = req.into_inner();
        let Json(data) = handle(Ok(Json(req.into()))).await?;
        Ok(Response::new(data.into()))
    }
}

impl From<ProbabilityRateResponse> for probabilityrate::ProbabilityRateResponse {
    fn from(value: ProbabilityRateResponse) -> Self {
        Self {
            roll_budget: value.roll_budget,
            data: value
                .data
                .into_iter()
                .map(|sims| probabilityrate::ReducedSims {
                    index: sims.into_iter().map(|e| e.into()).collect(),
                })
                .collect(),
        }
    }
}

impl From<ReducedSim> for probabilityrate::ReducedSim {
    fn from(value: ReducedSim) -> Self {
        let ReducedSim { eidolon, rate } = value;
        Self { eidolon, rate }
    }
}

impl From<probabilityrate::ProbabilityRatePayload> for ProbabilityRatePayload {
    fn from(value: probabilityrate::ProbabilityRatePayload) -> Self {
        ProbabilityRatePayload {
            current_eidolon: value.current_eidolon,
            pity: value.pity,
            pulls: value.pulls,
            next_guaranteed: value.next_guaranteed,
            enpitomized_pity: value.enpitomized_pity,
            banner: value.banner.try_into().unwrap(),
        }
    }
}

pub fn probabilityrate_route() -> MethodRouter {
    any_service(enable(ProbabilityRateServiceServer::new(
        ProbabilityRateResponse::default(),
    )))
}
