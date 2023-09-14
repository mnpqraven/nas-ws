use crate::handler::{error::WorkerError, FromAxumResponse};
use axum::Json;
use fake::Dummy;
use response_derive::JsonResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum_macros::EnumString;
use vercel_runtime::{Body, Response, StatusCode};

#[derive(Debug, Deserialize, Serialize, Clone, JsonResponse, JsonSchema, Dummy, Default)]
pub struct AssetPath(pub String);

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema, EnumString, Dummy)]
pub enum Element {
    Fire,
    Ice,
    Physical,
    Wind,
    #[serde(alias = "Thunder", alias = "Lightning")]
    #[strum(serialize = "Thunder", serialize = "Lightning")]
    Lightning,
    Quantum,
    Imaginary,
}

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, Copy, JsonSchema)]
pub enum Path {
    #[serde(alias = "Warrior")]
    Destruction,
    #[serde(alias = "Rogue")]
    Hunt,
    #[serde(alias = "Mage")]
    Erudition,
    #[serde(alias = "Shaman")]
    Harmony,
    #[serde(alias = "Warlock")]
    Nihility,
    #[serde(alias = "Knight")]
    Preservation,
    #[serde(alias = "Priest")]
    Abundance,
}

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema, Dummy)]
pub struct Attribute {
    field: String, // hp
    name: String,
    icon: AssetPath,
    value: f32,
    display: String,
    percent: bool,
}
#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
pub struct DbAttributeProperty {
    #[serde(rename = "type")]
    ttype: Property, // AttackAddedRatio
    field: String, // hp
    name: String,
    affix: bool,
    ratio: bool,
    icon: AssetPath,
    order: u32,
    percent: bool,
}
#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema, Dummy)]
pub struct AttributeProperty {
    #[serde(rename = "type")]
    ttype: Property, // AttackAddedRatio
    field: String, // hp
    name: String,
    icon: AssetPath,
    value: f32,
    display: String,
    percent: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Dummy, EnumString)]
pub enum Property {
    MaxHP,
    Attack,
    Defence,
    Speed,
    CriticalChance,
    CriticalDamage,
    BreakDamageAddedRatio,
    BreakDamageAddedRatioBase,
    HealRatio,
    MaxSP,
    SPRatio,
    StatusProbability,
    StatusResistance,
    CriticalChanceBase,
    CriticalDamageBase,
    HealRatioBase,
    StanceBreakAddedRatio,
    SPRatioBase,
    StatusProbabilityBase,
    StatusResistanceBase,
    PhysicalAddedRatio,
    PhysicalResistance,
    FireAddedRatio,
    FireResistance,
    IceAddedRatio,
    IceResistance,
    ThunderAddedRatio,
    ThunderResistance,
    WindAddedRatio,
    WindResistance,
    QuantumAddedRatio,
    QuantumResistance,
    ImaginaryAddedRatio,
    ImaginaryResistance,
    BaseHP,
    HPDelta,
    HPAddedRatio,
    BaseAttack,
    AttackDelta,
    AttackAddedRatio,
    BaseDefence,
    DefenceDelta,
    DefenceAddedRatio,
    BaseSpeed,
    HealTakenRatio,
    PhysicalResistanceDelta,
    FireResistanceDelta,
    IceResistanceDelta,
    ThunderResistanceDelta,
    WindResistanceDelta,
    QuantumResistanceDelta,
    ImaginaryResistanceDelta,
    SpeedDelta,
    SpeedAddedRatio,
    AllDamageTypeAddedRatio,
}

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema, Dummy)]
pub struct CharacterPath {
    id: String,
    name: String,
    icon: AssetPath,
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Element {
    pub fn color(&self) -> String {
        let color: &str = match self {
            Element::Fire => "#F84F36",
            Element::Ice => "#47C7FD",
            Element::Physical => "#FFFFFF",
            Element::Wind => "#00FF9C",
            Element::Lightning => "#8872F1",
            Element::Quantum => "#1C29BA",
            Element::Imaginary => "#F4D258",
        };
        color.to_string()
    }
    pub fn icon(&self) -> AssetPath {
        AssetPath(format!("icon/element/{}.png", self))
    }
}
