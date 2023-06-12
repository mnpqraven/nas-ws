use super::{gear::*, shared::*};
use crate::handler::{error::WorkerError, FromAxumResponse};
use axum::Json;
use response_derive::JsonResponse;
use serde::{Deserialize, Serialize};
use specta::{ts::*, *};
use vercel_runtime::{Body, Response, StatusCode};
use schemars::JsonSchema;

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
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

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
struct CharacterElement {
    id: String,
    name: Element,
    color: String,
    icon: AssetPath,
}

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
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

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
struct SkillTree {
    id: String,
    level: u32,
    icon: AssetPath,
}
