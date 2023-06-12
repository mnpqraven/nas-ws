use super::shared::*;
use crate::handler::{error::WorkerError, FromAxumResponse};
use axum::Json;
use response_derive::JsonResponse;
use serde::{Deserialize, Serialize};
use vercel_runtime::{Body, Response, StatusCode};
use schemars::JsonSchema;

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
pub struct LightCone {
    id: String,
    name: String,
    rarity: u32,
    rank: u32,
    level: u32,
    promotion: u32,
    icon: AssetPath,
    preview: AssetPath,
    portrait: AssetPath,
    path: CharacterPath,
    attributes: Vec<LightConeAttribute>,
    properties: Vec<LightConeProperty>,
}

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
pub struct Relic {
    id: String,
    name: String,
    set_id: String,
    set_name: String,
    rarity: u32,
    level: u32,
    icon: AssetPath,
    main_affix: MainAffix,
    sub_affix: Vec<SubAffix>,
}

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
pub struct RelicSet {
    id: String,
    name: String,
    icon: AssetPath,
    num: u32,
    desc: String,
    properties: Vec<RelicSetProperty>,
}

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
struct LightConeAttribute {
    field: String, // "hp" for enum
    name: String,
    icon: AssetPath,
    value: f32,
    display: String,
    percent: bool,
}
#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
struct LightConeProperty {
    #[serde(rename = "type")]
    ttype: String, // CriticalChanceBase for enum
    field: String,
    name: String,
    icon: AssetPath,
    value: f32,
    display: String,
    percent: bool,
}

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
struct MainAffix {
    #[serde(rename = "type")]
    ttype: String, // HPDelta
    field: String,
    name: String,
    icon: AssetPath,
    value: f32,
    display: String,
    percent: bool,
}

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
struct SubAffix {
    #[serde(rename = "type")]
    ttype: String, //DefenceDelta
    field: String,
    name: String,
    icon: AssetPath,
    value: f32,
    display: String,
    percent: bool,
    count: u32,
    step: u32,
}

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
struct RelicSetProperty {
    #[serde(rename = "type")]
    ttype: String, //QuantumAddedRatio
    field: String,
    name: String,
    icon: AssetPath,
    value: f32,
    display: String,
    percent: bool,
}
