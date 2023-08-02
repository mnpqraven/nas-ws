use crate::{
    handler::{error::WorkerError, FromAxumResponse},
    routes::honkai::banner::types::BannerType,
};
use axum::Json;
use response_derive::JsonResponse;
use serde::{Deserialize, Serialize};
use vercel_runtime::{Body, Response, StatusCode};

#[derive(Debug, Clone)]
pub struct Sim {
    pub eidolon: i32,
    pub rate: f64,
    pub pity: i32,
    pub guaranteed: bool,
    pub guaranteed_pity: i32,
}
#[derive(Debug, Serialize, JsonResponse, Clone)]
pub struct ReducedSim {
    pub eidolon: i32,
    pub rate: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ProbabilityRatePayload {
    pub current_eidolon: i32,
    pub pity: i32,
    pub pulls: i32,
    pub next_guaranteed: bool,
    pub enpitomized_pity: Option<i32>,
    pub banner: BannerType,
}

// master struct
#[derive(Debug, Serialize, JsonResponse, Clone, Default)]
pub struct ProbabilityRateResponse {
    pub roll_budget: i32,
    pub data: Vec<Vec<ReducedSim>>,
}
