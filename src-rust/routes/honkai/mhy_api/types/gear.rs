use super::shared::*;
use crate::handler::{error::WorkerError, FromAxumResponse};
use axum::Json;
use response_derive::JsonResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use vercel_runtime::{Body, Response, StatusCode};

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
pub struct LightCone {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    id: u32,
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
    #[serde(deserialize_with = "deserialize_number_from_string")]
    id: u32,
    name: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    set_id: u32, // enum mappable
    set_name: String,
    rarity: u32,
    level: u32,
    icon: AssetPath,
    main_affix: AffixProperty,
    sub_affix: Vec<SubAffix>,
}

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
pub struct RelicSet {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    id: u32,
    name: String,
    icon: AssetPath,
    num: u32,
    desc: String,
    properties: Vec<AffixProperty>,
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
struct AffixProperty {
    #[serde(rename = "type")]
    ttype: MainAffixType, // HPDelta
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
enum MainAffixType {
    HPDelta,     // hat only
    AttackDelta, // glove only
    // body
    HPAddedRatio,
    AttackAddedRatio,
    DefenceAddedRatio,
    CriticalChanceBase,
    CriticalDamageBase,
    HealRatioBase,
    StatusProbabilityBase, // EHR
    SpeedDelta,
    PhysicalAddedRatio,
    FireAddedRatio,
    IceAddedRatio,
    ThunderAddedRatio,
    WindAddedRatio,
    QuantumAddedRatio,
    ImaginaryAddedRatio,
    BreakDamageAddedRatioBase,
    SPRatioBase,
}

// HPDelta,     // hat only
// AttackDelta, // glove only
// // body
// HPAddedRatio,
// AttackAddedRatio,
// DefenceAddedRatio,
// CriticalChanceBase,
// CriticalDamageBase,
// HealRatioBase,
// StatusProbabilityBase, // EHR
// boots
// HPAddedRatio
// AttackAddedRatio
// DefenceAddedRatio
// SpeedDelta
//
// planar
// HPAddedRatio
// AttackAddedRatio
// DefenceAddedRatio
// PhysicalAddedRatio
// FireAddedRatio
// IceAddedRatio
// ThunderAddedRatio
// WindAddedRatio
// QuantumAddedRatio
// ImaginaryAddedRatio
//
// link robe
// BreakDamageAddedRatioBase
// SPRatioBase
// HPAddedRatio
// AttackAddedRatio
// DefenceAddedRatio
