use crate::handler::{error::WorkerError, FromAxumResponse};
use axum::Json;
use response_derive::JsonResponse;
use serde::{Deserialize, Serialize};
use vercel_runtime::{Body, Response, StatusCode};
use schemars::JsonSchema;

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
pub struct AssetPath(String);

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
pub enum Element {
    Fire,
    Ice,
    Physical,
    Wind,
    Lightning,
    Quantum,
    Imaginary,
}

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
pub struct Attribute {
    field: String, // hp
    name: String,
    icon: AssetPath,
    value: f32,
    display: String,
    percent: bool,
}
#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
pub struct AttributeProperty {
    #[serde(rename = "type")]
    ttype: String, // AttackAddedRatio
    field: String, // hp
    name: String,
    icon: AssetPath,
    value: f32,
    display: String,
    percent: bool,
}

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
pub struct CharacterPath {
    id: String,
    name: String,
    icon: AssetPath,
}
