use crate::builder::{get_db_client, traits::DbAction};
use crate::handler::{error::WorkerError, FromAxumResponse};
use async_trait::async_trait;
use axum::Json;
use fake::Dummy;
use libsql_client::{args, Statement};
use response_derive::JsonResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use vercel_runtime::{Body, Response, StatusCode};

#[derive(Serialize, Deserialize)]
pub struct TextMap(pub HashMap<String, String>);

#[derive(Debug, Serialize, Deserialize, Clone, Copy, JsonSchema)]
pub struct Param {
    #[serde(alias = "Value")]
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum LightConeRarity {
    CombatPowerLightconeRarity3 = 3,
    CombatPowerLightconeRarity4 = 4,
    CombatPowerLightconeRarity5 = 5,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename(serialize = "camelCase"))]
pub struct AbilityProperty {
    #[serde(alias = "PropertyType")]
    property_type: Property,
    #[serde(alias = "Value")]
    value: Param,
}

impl From<Param> for f64 {
    fn from(val: Param) -> Self {
        val.value
    }
}

#[derive(Debug, Display, Serialize, Deserialize, Clone, JsonSchema, EnumString)]
pub enum Anchor {
    Point01,
    Point02,
    Point03,
    Point04,
    Point05,
    Point06,
    Point07,
    Point08,
    Point09,
    Point10,
    Point11,
    Point12,
    Point13,
    Point14,
    Point15,
    Point16,
    Point17,
    Point18,
}

#[derive(
    Debug,
    Display,
    Serialize,
    Deserialize,
    Clone,
    JsonSchema,
    Eq,
    PartialEq,
    Copy,
    EnumString,
    EnumIter,
)]

pub enum SkillType {
    // id listing should always be in this order
    Normal,     // basic attack
    BPSkill,    // Skill
    Ultra,      // Ultimate
    Talent,     // Talent
    MazeNormal, // overworld normal
    Maze,       // overworld Technique
}

#[async_trait]
impl DbAction for SkillType {
    async fn seed() -> Result<(), WorkerError> {
        let client = get_db_client().await?;
        let st: Vec<Statement> = SkillType::iter()
            .enumerate()
            .map(|(index, ttype)| {
                Statement::with_args(
                    "INSERT OR REPLACE INTO honkai_skillType (name, type) VALUES (?, ?)",
                    args!(ttype.to_string(), index),
                )
            })
            .collect();
        client.batch(st).await?;
        Ok(())
    }

    async fn teardown() -> Result<(), WorkerError> {
        let client = get_db_client().await?;
        client.execute("DELETE FROM honkai_skillType").await?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonResponse, JsonSchema, Dummy, Default)]
pub struct AssetPath(pub String);

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
                    "INSERT OR REPLACE INTO honkai_element VALUES (?, ?)",
                    args!(element.to_string(), element as i32),
                )
            })
            .collect();

        client.batch(st).await?;
        Ok(())
    }
    async fn teardown() -> Result<(), WorkerError> {
        let client = get_db_client().await?;
        client.execute("DELETE FROM honkai_element").await?;
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
                    "INSERT OR REPLACE INTO honkai_path (name, type) VALUES (?, ?)",
                    args!(path.to_string(), path as i32),
                )
            })
            .collect();

        client.batch(st).await?;
        Ok(())
    }

    async fn teardown() -> Result<(), WorkerError> {
        let client = get_db_client().await?;
        client.execute("DELETE FROM honkai_path").await?;
        Ok(())
    }
}
