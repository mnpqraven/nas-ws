use std::{num::ParseIntError, str::FromStr};

use crate::routes::honkai::mhy_api::types::shared::{AssetPath, Element, Path};
use serde::{Deserialize, Serialize};
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
#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    id: u32,
    pub(super) name: String,
    tag: String,
    rarity: u8,
    path: Path,
    element: Element,
    max_sp: u32,
    ranks: Vec<String>,
    skills: Vec<String>,
    skill_trees: Vec<String>,
    pub(super) icon: AssetPath,
    preview: AssetPath,
    portrait: AssetPath,
    #[serde(skip)]
    guide_overview: Vec<String>,
    #[serde(skip)]
    guide_material: Vec<String>,
    #[serde(skip)]
    guide_evaluation: Vec<String>,
}
