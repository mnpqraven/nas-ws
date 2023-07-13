use crate::routes::honkai::{
    dm_api::{hash::TextHash, types::LightConeRarity},
    mhy_api::types_parsed::shared::{AssetPath, Path},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct UpstreamEquipmentConfig {
    #[serde(alias = "EquipmentID")]
    pub equipment_id: u32,
    #[serde(alias = "Release")]
    pub release: bool,
    #[serde(alias = "EquipmentName")]
    pub equipment_name: TextHash,
    #[serde(alias = "EquipmentDesc")]
    pub equipment_desc: TextHash,
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
pub struct EquipmentConfig {
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
