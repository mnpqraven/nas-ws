use crate::routes::honkai::mhy_api::types_parsed::shared::{AssetPath, Path, Property};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct TextMap(pub HashMap<String, String>);

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
    pub icon_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct Param {
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct Hash {
    pub hash: i64,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EquipmentConfig {
    #[serde(alias = "EquipmentID")]
    pub equipment_id: u32,
    pub release: bool,
    pub equipment_name: Hash,
    pub equipment_desc: Hash, // WARN: HASH LEADING TO NONE
    pub rarity: LightConeRarity,
    pub avatar_base_type: Path,
    pub max_promotion: u32,
    pub max_rank: u32,
    pub exp_type: u32,
    #[serde(alias = "SkillID")]
    pub skill_id: u32,
    pub exp_provide: u32,
    pub coin_cost: u32,
    pub rank_up_cost_list: Vec<u32>,
    #[serde(skip)]
    pub thumbnail_path: AssetPath,
    #[serde(skip)]
    pub image_path: AssetPath,
    #[serde(skip)]
    pub item_right_panel_offset: Vec<f32>,
    #[serde(skip)]
    pub avatar_detail_offset: Vec<f32>,
    #[serde(skip)]
    pub battle_dialog_offset: Vec<f32>,
    #[serde(skip)]
    pub gacha_result_offset: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct EquipmentConfigMerged {
    #[serde(alias = "EquipmentID")]
    pub equipment_id: u32,
    pub release: bool,
    pub equipment_name: String,
    pub equipment_desc: String, // WARN: HASH LEADING TO NONE
    pub rarity: u8,
    pub avatar_base_type: Path,
    pub max_promotion: u32,
    pub max_rank: u32,
    pub exp_type: u32,
    #[serde(alias = "SkillID")]
    pub skill_id: u32,
    pub exp_provide: u32,
    pub coin_cost: u32,
    pub rank_up_cost_list: Vec<u32>,
    #[serde(skip)]
    pub thumbnail_path: AssetPath,
    #[serde(skip)]
    pub image_path: AssetPath,
    #[serde(skip)]
    pub item_right_panel_offset: Vec<f32>,
    #[serde(skip)]
    pub avatar_detail_offset: Vec<f32>,
    #[serde(skip)]
    pub battle_dialog_offset: Vec<f32>,
    #[serde(skip)]
    pub gacha_result_offset: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum LightConeRarity {
    CombatPowerLightconeRarity3 = 3,
    CombatPowerLightconeRarity4 = 4,
    CombatPowerLightconeRarity5 = 5,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AbilityProperty {
    property_type: Property,
    value: Param,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EquipmentSkillConfig {
    #[serde(alias = "SkillID")]
    pub skill_id: u32,
    pub skill_name: Hash,
    pub skill_desc: Hash,
    pub level: u32,
    pub ability_name: String,
    pub param_list: Vec<Param>,
    pub ability_property: Vec<AbilityProperty>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct EquipmentSkillConfigMerged {
    #[serde(alias = "SkillID")]
    pub skill_id: u32,
    /// merge
    pub skill_name: String,
    pub skill_desc: String,
    /// merge
    pub level: Vec<u32>,
    pub ability_name: String,
    /// merge
    pub param_list: Vec<Vec<Param>>,
    /// merge
    pub ability_property: Vec<Vec<AbilityProperty>>,
}
