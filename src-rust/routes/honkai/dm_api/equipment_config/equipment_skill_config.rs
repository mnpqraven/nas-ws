use crate::routes::honkai::dm_api::{
    desc_param::ParameterizedDescription,
    hash::TextHash,
    types::{AbilityProperty, Param},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpstreamEquipmentSkillConfig {
    #[serde(alias = "SkillID")]
    pub skill_id: u32,
    #[serde(alias = "SkillName")]
    pub skill_name: TextHash,
    #[serde(alias = "SkillDesc")]
    pub skill_desc: TextHash,
    #[serde(alias = "Level")]
    pub level: u32,
    #[serde(alias = "AbilityName")]
    pub ability_name: String,
    #[serde(alias = "ParamList")]
    pub param_list: Vec<Param>,
    #[serde(alias = "AbilityProperty")]
    pub ability_property: Vec<AbilityProperty>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct EquipmentSkillConfig {
    #[serde(alias = "SkillID")]
    pub skill_id: u32,
    /// merge
    #[serde(alias = "SkillName")]
    pub skill_name: String,
    #[serde(alias = "SkillDesc")]
    pub skill_desc: ParameterizedDescription,
    /// merge
    #[serde(skip, alias = "Level")]
    pub level: Vec<u32>,
    #[serde(alias = "AbilityName")]
    pub ability_name: String,
    /// merge
    #[serde(alias = "ParamList")]
    pub param_list: Vec<Vec<String>>,
    /// merge
    #[serde(alias = "AbilityProperty")]
    pub ability_property: Vec<Vec<AbilityProperty>>,
}
