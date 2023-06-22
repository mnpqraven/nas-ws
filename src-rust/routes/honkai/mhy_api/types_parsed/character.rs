use super::{gear::*, shared::*};
use crate::handler::{error::WorkerError, FromAxumResponse};
use axum::Json;
use fake::Dummy;
use response_derive::JsonResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vercel_runtime::{Body, Response, StatusCode};

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema, Dummy)]
pub struct Character {
    id: String,
    name: String,
    rarity: u32,
    rank: u32,
    level: u32,
    promotion: u32,
    icon: AssetPath,
    preview: AssetPath,
    portrait: AssetPath,
    rank_icons: Vec<AssetPath>,
    path: CharacterPath,
    element: CharacterElement,
    skills: Vec<Skill>,
    skill_trees: Vec<SkillTree>,
    light_cone: LightCone,
    relics: Vec<Relic>,
    relic_sets: Vec<RelicSet>,
    attributes: Vec<Attribute>,
    additions: Vec<Attribute>,
    properties: Vec<AttributeProperty>,
}

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema, Dummy)]
pub struct CharacterElement {
    id: String,
    name: Element,
    color: String,
    icon: AssetPath,
}

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema, Dummy)]
struct Skill {
    id: String,
    name: String,
    level: u32,
    max_level: u32,
    element: Option<CharacterElement>,
    #[serde(rename = "type")]
    ttype: String, // "Normal" for enum
    type_text: String,
    effect: String,
    effect_text: String,
    simple_desc: String,
    desc: String,
    icon: AssetPath,
}

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema, Dummy)]
struct SkillTree {
    id: String,
    level: u32,
    icon: AssetPath,
}

impl From<Element> for CharacterElement {
    fn from(value: Element) -> Self {
        match value {
            Element::Fire => Self {
                id: "Fire".into(),
                name: Element::Fire,
                color: "#F84F36".into(),
                icon: AssetPath("icon/element/Fire.png".into()),
            },
            Element::Ice => Self {
                id: "Ice".into(),
                name: Element::Ice,
                color: "#47C7FD".into(),
                icon: AssetPath("icon/element/Ice.png".into()),
            },
            Element::Physical => Self {
                id: "Physical".into(),
                name: Element::Physical,
                color: "#FFFFFF".into(),
                icon: AssetPath("icon/element/Physical.png".into()),
            },
            Element::Wind => Self {
                id: "Wind".into(),
                name: Element::Wind,
                color: "#00FF9C".into(),
                icon: AssetPath("icon/element/Wind.png".into()),
            },
            Element::Lightning => Self {
                id: "Lightning".into(),
                name: Element::Lightning,
                color: "#8872F1".into(),
                icon: AssetPath("icon/element/Lightning.png".into()),
            },
            Element::Quantum => Self {
                id: "Quantum".into(),
                name: Element::Quantum,
                color: "#1C29BA".into(),
                icon: AssetPath("icon/element/Quantum.png".into()),
            },
            Element::Imaginary => Self {
                id: "Imaginary".into(),
                name: Element::Imaginary,
                color: "#F4D258".into(),
                icon: AssetPath("icon/element/Imaginary.png".into()),
            },
        }
    }
}
