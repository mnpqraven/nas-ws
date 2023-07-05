use super::desc_param::ParameterizedDescription;
use crate::routes::honkai::mhy_api::types_parsed::shared::{AssetPath, Path, Property};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct TextMap(pub HashMap<String, String>);

#[derive(Serialize, Deserialize)]
pub struct SkillTreeConfigWrapper(pub HashMap<String, SkillTreeConfig>);

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

#[derive(Debug, Serialize, Deserialize, Clone, Copy, JsonSchema)]
pub struct Param {
    #[serde(alias = "Value")]
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Hash {
    #[serde(alias = "Hash")]
    pub hash: i64,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct EquipmentConfig {
    #[serde(alias = "EquipmentID")]
    pub equipment_id: u32,
    #[serde(alias = "Release")]
    pub release: bool,
    #[serde(alias = "EquipmentName")]
    pub equipment_name: Hash,
    #[serde(alias = "EquipmentDesc")]
    pub equipment_desc: Hash, // WARN: HASH LEADING TO NONE
    #[serde(alias = "Rarity")]
    pub rarity: LightConeRarity,
    #[serde(alias = "AvatarBaseType")]
    pub avatar_base_type: Path,
    #[serde(alias = "MaxPromotion")]
    pub max_promotion: u32,
    #[serde(alias = "MaxRank")]
    pub max_rank: u32,
    #[serde(alias = "ExpType")]
    pub exp_type: u32,
    #[serde(alias = "SkillID")]
    pub skill_id: u32,
    #[serde(alias = "ExpProvide")]
    pub exp_provide: u32,
    #[serde(alias = "CoinCost")]
    pub coin_cost: u32,
    #[serde(alias = "RankUpCostList")]
    pub rank_up_cost_list: Vec<u32>,
    #[serde(skip)]
    #[serde(alias = "ThumbnailPath")]
    pub thumbnail_path: AssetPath,
    #[serde(skip)]
    #[serde(alias = "ImagePath")]
    pub image_path: AssetPath,
    #[serde(skip)]
    #[serde(alias = "ItemRightPanelOffset")]
    pub item_right_panel_offset: Vec<f32>,
    #[serde(skip)]
    #[serde(alias = "AvatarDetailOffset")]
    pub avatar_detail_offset: Vec<f32>,
    #[serde(skip)]
    #[serde(alias = "BattleDialogOffset")]
    pub battle_dialog_offset: Vec<f32>,
    #[serde(skip)]
    #[serde(alias = "GachaResultOffset")]
    pub gacha_result_offset: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct EquipmentConfigMerged {
    #[serde(alias = "EquipmentID")]
    pub equipment_id: u32,
    #[serde(alias = "Release")]
    pub release: bool,
    #[serde(alias = "EquipmentName")]
    pub equipment_name: String,
    #[serde(alias = "EquipmentDesc")]
    pub equipment_desc: String, // WARN: HASH LEADING TO NONE
    #[serde(alias = "Rarity")]
    pub rarity: u8,
    #[serde(alias = "AvatarBaseType")]
    pub avatar_base_type: Path,
    #[serde(alias = "MaxPromotion")]
    pub max_promotion: u32,
    #[serde(alias = "MaxRank")]
    pub max_rank: u32,
    #[serde(alias = "ExpType")]
    pub exp_type: u32,
    #[serde(alias = "SkillID")]
    pub skill_id: u32,
    #[serde(alias = "ExpProvide")]
    pub exp_provide: u32,
    #[serde(alias = "CoinCost")]
    pub coin_cost: u32,
    #[serde(alias = "RankUpCostList")]
    pub rank_up_cost_list: Vec<u32>,
    #[serde(skip, alias = "ThumbnailPath")]
    pub thumbnail_path: AssetPath,
    #[serde(skip, alias = "ImagePath")]
    pub image_path: AssetPath,
    #[serde(skip, alias = "ItemRightPanelOffset")]
    pub item_right_panel_offset: Vec<f32>,
    #[serde(skip, alias = "AvatarDetailOffset")]
    pub avatar_detail_offset: Vec<f32>,
    #[serde(skip, alias = "BattleDialogOffset")]
    pub battle_dialog_offset: Vec<f32>,
    #[serde(skip, alias = "GachaResultOffset")]
    pub gacha_result_offset: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum LightConeRarity {
    CombatPowerLightconeRarity3 = 3,
    CombatPowerLightconeRarity4 = 4,
    CombatPowerLightconeRarity5 = 5,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AbilityProperty {
    #[serde(alias = "PropertyType")]
    property_type: Property,
    #[serde(alias = "Value")]
    value: Param,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EquipmentSkillConfig {
    #[serde(alias = "SkillID")]
    pub skill_id: u32,
    #[serde(alias = "SkillName")]
    pub skill_name: Hash,
    #[serde(alias = "SkillDesc")]
    pub skill_desc: Hash,
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
pub struct EquipmentSkillConfigMerged {
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
