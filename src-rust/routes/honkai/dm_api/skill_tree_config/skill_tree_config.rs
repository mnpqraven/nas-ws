use crate::routes::honkai::dm_api::types::Param;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillTreeConfig {
    #[serde(alias = "PointID")]
    point_id: u32,
    #[serde(alias = "Anchor")]
    anchor: String,
    #[serde(alias = "PointName")]
    pub point_name: String,
    #[serde(alias = "PointDesc")]
    pub point_desc: String,
    #[serde(alias = "ParamList")]
    pub param_list: Vec<Param>,
    #[serde(alias = "IconPath")]
    pub icon_path: String,
}
