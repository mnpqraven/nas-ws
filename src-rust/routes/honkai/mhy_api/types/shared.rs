use crate::handler::{error::WorkerError, FromAxumResponse};
use axum::Json;
use response_derive::JsonResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum_macros::EnumString;
use vercel_runtime::{Body, Response, StatusCode};

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
pub struct AssetPath(pub String);

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema, EnumString)]
pub enum Element {
    Fire,
    Ice,
    Physical,
    Wind,
    #[serde(alias = "Thunder")]
    #[strum(serialize = "Thunder")]
    Lightning,
    Quantum,
    Imaginary,
}

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
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
