use crate::{
    builder::{traits::DbAction, get_db_client},
    handler::{error::WorkerError, FromAxumResponse},
};
use async_trait::async_trait;
use axum::Json;
use fake::Dummy;
use libsql_client::{Statement, args};
use response_derive::JsonResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use vercel_runtime::{Body, Response, StatusCode};

#[derive(Debug, Deserialize, Serialize, Clone, JsonResponse, JsonSchema, Dummy, Default)]
pub struct AssetPath(pub String);

#[derive(
    Debug,
    Deserialize,
    Serialize,
    JsonResponse,
    Clone,
    Copy,
    JsonSchema,
    EnumString,
    Dummy,
    EnumIter,
)]
pub enum Element {
    Fire = 0,
    Ice = 1,
    Physical = 2,
    Wind = 3,
    #[serde(alias = "Thunder", alias = "Lightning")]
    #[strum(serialize = "Thunder", serialize = "Lightning")]
    Lightning = 4,
    Quantum = 5,
    Imaginary = 6,
}

#[derive(
    Debug,
    Display,
    Deserialize,
    Serialize,
    JsonResponse,
    Clone,
    Copy,
    JsonSchema,
    EnumString,
    EnumIter,
)]
pub enum Path {
    #[serde(alias = "Warrior")]
    Destruction = 0,
    #[serde(alias = "Rogue")]
    Hunt = 1,
    #[serde(alias = "Mage")]
    Erudition = 2,
    #[serde(alias = "Shaman")]
    Harmony = 3,
    #[serde(alias = "Warlock")]
    Nihility = 4,
    #[serde(alias = "Knight")]
    Preservation = 5,
    #[serde(alias = "Priest")]
    Abundance = 6,
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

#[async_trait]
impl DbAction for Element {
    async fn seed() -> Result<(), WorkerError> {
        let client = get_db_client().await?;
        let st: Vec<Statement> = Element::iter()
            .map(|element| {
                Statement::with_args(
                    "INSERT OR REPLACE INTO element VALUES (?, ?)",
                    args!(element.to_string(), element as i32),
                )
            })
            .collect();

        client.batch(st).await?;
        Ok(())
    }
}

#[async_trait]
impl DbAction for Path {
    async fn seed() -> Result<(), WorkerError> {
        let client = get_db_client().await?;
        let st: Vec<Statement> = Path::iter()
            .map(|path| {
                Statement::with_args(
                    "INSERT OR REPLACE INTO path VALUES (?, ?)",
                    args!(path.to_string(), path as i32),
                )
            })
            .collect();

        client.batch(st).await?;
        Ok(())
    }
}