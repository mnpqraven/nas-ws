use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SkillTreeConfigWrapper(pub HashMap<String, SkillTreeConfig>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SkillTreeConfig {
    #[serde(alias = "PointID")]
    point_id: u32,
    anchor: String,
    pub point_name: String,
    pub point_desc: String,
    pub param_list: Vec<Param>,
    pub icon_path: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Param {
    pub value: f32,
}
