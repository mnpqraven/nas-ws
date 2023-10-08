use crate::builder::get_db_client;
use crate::builder::traits::DbAction;
use crate::routes::honkai::mhy_api::WorkerError;
use crate::{handler::FromAxumResponse, routes::honkai::mhy_api::types_parsed::shared::Property};
use async_trait::async_trait;
use axum::Json;
use core::fmt;
use libsql_client::{args, Statement};
use response_derive::JsonResponse;
use schemars::JsonSchema;
use std::{fmt::Display, marker::PhantomData, num::ParseIntError, str::FromStr, sync::Arc};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use vercel_runtime::{Body, Response, StatusCode};

use crate::routes::honkai::mhy_api::types_parsed::shared::{AssetPath, Element, Path};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};
use serde_aux::prelude::*;
use serde_repr::Deserialize_repr;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(super) struct RelicSet {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    id: RelicSetId,
    name: String,
    desc: Vec<String>,
    properties: Vec<Vec<SetProperty>>,
    icon: AssetPath,
    #[allow(dead_code)]
    #[serde(skip)]
    guide_overview: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(super) struct RelicInfo {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    id: u32,
    set_id: String,
    name: String,
    rarity: u32,
    #[serde(rename = "type")]
    ttype: RelicType,
    max_level: u32,
    main_affix_id: String,
    sub_affix_id: String,
    icon: AssetPath,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct SetProperty {
    #[serde(rename = "type")]
    ttype: String,
    value: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum RelicType {
    Head,
    Hand,
    Body,
    Foot,
    Object, // planar sphere
    Neck,   // link robe
}

#[derive(Debug, Deserialize_repr)]
#[repr(u32)]
enum RelicSetId {
    Passerby = 101,
    Musketeer = 102,
    Knight = 103,
    Hunter = 104,
    Champion = 105,
    Wuthering = 106,
    Firesmith = 107,
    Genius = 108,
    Sizzling = 109,
    Eagle = 110,
    Thief = 111,
    Wastelander = 112,
    Space = 301,
    Fleet = 302,
    PanGalactic = 303,
    Belobog = 304,
    Differentiator = 305,
    Salsotto = 306,
    Talia = 307,
    Vonwacq = 308,
}

impl FromStr for RelicSetId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = u32::from_str(s)?;
        let t = unsafe { std::mem::transmute(id) };
        Ok(t)
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, JsonResponse, JsonSchema)]
pub struct DbCharacter {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u32,
    pub name: String,
    tag: String,
    pub rarity: u8,
    path: Path,
    pub element: Element,
    pub max_sp: u32,
    ranks: Vec<String>,
    /// skillIds
    pub skills: Vec<String>,
    pub skill_trees: Vec<String>,
    pub icon: AssetPath,
    preview: AssetPath,
    portrait: AssetPath,
    #[serde(skip)]
    guide_overview: Vec<String>,
    #[serde(skip)]
    guide_material: Vec<String>,
    #[serde(skip)]
    guide_evaluation: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct DbCharacterSkill {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u32, // characterId + skillId
    pub name: String,
    max_level: u32,
    #[serde(deserialize_with = "string_empty_as_none")]
    element: Option<Element>,
    #[serde(rename = "type")]
    pub ttype: SkillType,
    type_text: String,
    effect: String,
    effect_text: String,
    simple_desc: String,
    pub desc: ParameterizedFmt,
    pub params: Arc<[Parameter]>,
    icon: AssetPath,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Parameter(pub Arc<[f64]>);

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct DbCharacterSkillTree {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    id: u32,
    max_level: u32,
    anchor: Anchor, // point13
    pre_points: Vec<String>,
    level_up_skills: Vec<SkillKV>,
    levels: Vec<SkillLevel>,
    pub icon: AssetPath,
}
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct DbCharacterEidolon {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    id: u32,
    name: String,
    rank: u32,
    desc: String,
    materials: Vec<MaterialKV>,
    level_up_skills: Vec<SkillKV>,
    icon: AssetPath,
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

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
struct SkillLevel {
    promotion: u32,
    properties: Vec<PropertyKV>,
    materials: Vec<MaterialKV>,
}
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
struct PropertyKV {
    #[serde(alias = "type")]
    ttype: Property, // ICEADDEDRATIO
    value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
struct SkillKV {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    id: u32,
    num: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
struct MaterialKV {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    id: u32,
    num: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ParameterizedFmt(pub String);

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
                    "INSERT OR REPLACE INTO skillType (name, type) VALUES (?, ?)",
                    args!(ttype.to_string(), index),
                )
            })
            .collect();
        client.batch(st).await?;
        Ok(())
    }
}

/// https://tikv.github.io/doc/src/serde_with/rust.rs.html#874-940
pub fn string_empty_as_none<'de, D, S>(deserializer: D) -> Result<Option<S>, D::Error>
where
    D: Deserializer<'de>,
    S: FromStr,
    S::Err: Display,
{
    struct OptionStringEmptyNone<S>(PhantomData<S>);
    impl<'de, S> Visitor<'de> for OptionStringEmptyNone<S>
    where
        S: FromStr,
        S::Err: Display,
    {
        type Value = Option<S>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("any string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match value {
                "" => Ok(None),
                v => S::from_str(v).map(Some).map_err(de::Error::custom),
            }
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match &*value {
                "" => Ok(None),
                v => S::from_str(v).map(Some).map_err(de::Error::custom),
            }
        }

        // handles the `null` case
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_any(OptionStringEmptyNone(PhantomData))
}
