use crate::{
    builder::{get_db_client, traits::DbAction},
    handler::error::WorkerError,
    routes::honkai::{
        dm_api::{
            hash::TextHash,
            types::{AssetPath, LightConeRarity, Path, TextMap},
        },
        traits::DbData,
    },
};
use async_trait::async_trait;
use libsql_client::{args, Statement};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

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

/// metadata for light cones
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct EquipmentConfig {
    pub equipment_id: u32,
    pub release: bool,
    pub equipment_name: String,
    pub equipment_desc: String, // WARN: HASH LEADING TO NONE
    pub rarity: u8,
    pub avatar_base_type: Path,
    pub max_promotion: u32,
    pub max_rank: u32,
    pub exp_type: u32,
    pub skill_id: u32,
    pub exp_provide: u32,
    pub coin_cost: u32,
    pub rank_up_cost_list: Vec<u32>,
    pub thumbnail_path: AssetPath,
    pub image_path: AssetPath,
    pub item_right_panel_offset: Vec<f32>,
    pub avatar_detail_offset: Vec<f32>,
    pub battle_dialog_offset: Vec<f32>,
    pub gacha_result_offset: Vec<f32>,
}

impl UpstreamEquipmentConfig {
    fn to_merged(&self, (equipment_name, equipment_desc): (String, String)) -> EquipmentConfig {
        EquipmentConfig {
            equipment_id: self.equipment_id,
            release: self.release,
            equipment_name,
            equipment_desc,
            rarity: self.rarity as u8,
            avatar_base_type: self.avatar_base_type,
            max_promotion: self.max_promotion,
            max_rank: self.max_rank,
            exp_type: self.exp_type,
            skill_id: self.skill_id,
            exp_provide: self.exp_provide,
            coin_cost: self.coin_cost,
            rank_up_cost_list: self.rank_up_cost_list.clone(),
            thumbnail_path: self.thumbnail_path.clone(),
            image_path: self.image_path.clone(),
            item_right_panel_offset: self.item_right_panel_offset.clone(),
            avatar_detail_offset: self.avatar_detail_offset.clone(),
            battle_dialog_offset: self.battle_dialog_offset.clone(),
            gacha_result_offset: self.gacha_result_offset.clone(),
        }
    }
}

#[async_trait]
impl DbData for EquipmentConfig {
    type TUpstream = HashMap<u32, UpstreamEquipmentConfig>;
    type TLocal = HashMap<u32, EquipmentConfig>;

    fn path_data() -> &'static str {
        "ExcelOutput/EquipmentConfig.json"
    }

    async fn upstream_convert(
        from: HashMap<u32, UpstreamEquipmentConfig>,
    ) -> Result<HashMap<u32, EquipmentConfig>, WorkerError> {
        let text_map: HashMap<String, String> = TextMap::read().await?;
        let arced_text_map = Arc::new(text_map);

        let transformed: HashMap<u32, EquipmentConfig> = from
            .into_iter()
            .map(|(k, value)| {
                let arced_chunk = arced_text_map.clone();
                let get_value = move |key: &str| arced_chunk.get(key).cloned().unwrap_or_default();

                let (eq_name, eq_desc) = (
                    get_value(&value.equipment_name.hash.to_string()),
                    get_value(&value.equipment_desc.hash.to_string()),
                );
                let v = value.to_merged((eq_name, eq_desc));
                (k, v)
            })
            .collect();
        Ok(transformed)
    }
}

#[async_trait]
impl DbAction for EquipmentConfig {
    async fn seed() -> Result<(), WorkerError> {
        let client = get_db_client().await?;
        let lc_db = EquipmentConfig::read().await?;
        let sts = lc_db
            .into_values()
            .map(|lc| {
                let EquipmentConfig {
                    equipment_id,
                    release,
                    equipment_name,
                    rarity,
                    avatar_base_type,
                    max_promotion,
                    max_rank,
                    skill_id,
                    ..
                } = lc;

                Statement::with_args(
                    "INSERT OR REPLACE INTO
                    honkai_lightCone (
                        id, release, name, rarity, path, max_promotion,
                        max_rank, skill_id
                    ) VALUES (?,?,?,?,?,?,?,?)",
                    args!(
                        equipment_id,
                        release as i32,
                        equipment_name,
                        rarity,
                        avatar_base_type.to_string(),
                        max_promotion,
                        max_rank,
                        skill_id
                    ),
                )
            })
            .collect::<Vec<Statement>>();

        client.batch(sts).await?;

        Ok(())
    }
    async fn teardown() -> Result<(), WorkerError> {
        let client = get_db_client().await?;
        client.execute("DELETE FROM honkai_lightCone").await?;
        Ok(())
    }
}
