use self::{character::*, player::*};
use crate::handler::FromAxumResponse;
use crate::routes::honkai::mhy_api::WorkerError;
use axum::Json;
use response_derive::JsonResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use specta::{ts::*, *};
use vercel_runtime::{Body, Response, StatusCode};

pub mod character;
pub mod gear;
pub mod player;
pub mod shared;

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
pub struct MihoResponse {
    player: Player,
    // NOTE: use this to omit data being sent to the frontend
    // #[serde(skip_serializing)]
    characters: Vec<Character>,
}
