use std::collections::{BTreeMap, HashMap};

use crate::{
    handler::error::WorkerError,
    routes::honkai::{
        dm_api::{avatar_config::upstream_avatar_config::Item, types::Param},
        traits::{DbData, DbDataLike},
    },
};
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(target_os = "windows")]
pub const EQUIPMENT_PROMOTION_CONFIG_LOCAL: &str = "c:\\tmp\\equipment_promotion_config.json";
#[cfg(target_os = "linux")]
pub const EQUIPMENT_PROMOTION_CONFIG_LOCAL: &str = "/tmp/equipment_promotion_config.json";

pub const EQUIPMENT_PROMOTION_CONFIG_REMOTE: &str =
    "https://raw.githubusercontent.com/Dimbreath/StarRailData/master/ExcelOutput/EquipmentPromotionConfig.json";

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
struct UpstreamEquipmentPromotionConfig {
    #[serde(alias = "EquipmentID")]
    equipment_id: u32,
    #[serde(alias = "Promotion")]
    promotion: u32,
    #[serde(alias = "PromotionCostList")]
    promotion_cost_list: Vec<Item>,
    #[serde(alias = "WorldLevelRequire")]
    world_level_require: Option<u32>,
    #[serde(alias = "MaxLevel")]
    max_level: u32,
    #[serde(alias = "BaseHP")]
    base_hp: Param,
    #[serde(alias = "BaseHPAdd")]
    base_hpadd: Param,
    #[serde(alias = "BaseAttack")]
    base_attack: Param,
    #[serde(alias = "BaseAttackAdd")]
    base_attack_add: Param,
    #[serde(alias = "BaseDefence")]
    base_defence: Param,
    #[serde(alias = "BaseDefenceAdd")]
    base_defence_add: Param,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Default)]
pub struct EquipmentPromotionConfig {
    #[serde(alias = "EquipmentID")]
    pub equipment_id: u32,
    #[serde(alias = "Promotion")]
    pub promotion: Vec<u32>,
    #[serde(alias = "PromotionCostList")]
    pub promotion_cost_list: Vec<Vec<Item>>,
    #[serde(alias = "WorldLevelRequire")]
    pub world_level_require: Vec<u32>,
    #[serde(alias = "MaxLevel")]
    pub max_level: Vec<u32>,
    #[serde(alias = "BaseHP")]
    pub base_hp: Vec<f64>,
    #[serde(alias = "BaseHPAdd")]
    pub base_hpadd: Vec<f64>,
    #[serde(alias = "BaseAttack")]
    pub base_attack: Vec<f64>,
    #[serde(alias = "BaseAttackAdd")]
    pub base_attack_add: Vec<f64>,
    #[serde(alias = "BaseDefence")]
    pub base_defence: Vec<f64>,
    #[serde(alias = "BaseDefenceAdd")]
    pub base_defence_add: Vec<f64>,
}

#[async_trait]
impl<T: DbDataLike> DbData<T> for EquipmentPromotionConfig {
    fn path_data() -> (&'static str, &'static str) {
        (
            EQUIPMENT_PROMOTION_CONFIG_LOCAL,
            EQUIPMENT_PROMOTION_CONFIG_REMOTE,
        )
    }

    async fn try_write_disk() -> Result<String, WorkerError> {
        let (local_path, fallback_url) = <EquipmentPromotionConfig as DbData<T>>::path_data();
        let data = reqwest::get(fallback_url).await?.text().await?;
        let data: HashMap<String, BTreeMap<u32, UpstreamEquipmentPromotionConfig>> =
            serde_json::from_str(&data)?;

        let merged: HashMap<String, EquipmentPromotionConfig> = data
            .into_iter()
            .map(|(k, inner_map)| {
                let mut res = EquipmentPromotionConfig {
                    equipment_id: k.parse::<u32>().unwrap(),
                    ..Default::default()
                };
                for (key, value) in inner_map.into_iter() {
                    res.promotion.push(value.promotion);
                    res.promotion_cost_list.push(value.promotion_cost_list);
                    res.world_level_require.push(value.world_level_require.unwrap_or_default());
                    res.max_level.push(value.max_level);
                    res.base_hp.push(value.base_hp.value);
                    res.base_hpadd.push(value.base_hpadd.value);
                    res.base_attack.push(value.base_attack.value);
                    res.base_attack_add.push(value.base_attack_add.value);
                    res.base_defence.push(value.base_defence.value);
                    res.base_defence_add.push(value.base_defence_add.value);
                }

                (k, res)
            })
            .collect();

        let result_text = serde_json::to_string_pretty(&merged)?;
        std::fs::write(local_path, &result_text)?;
        Ok(result_text)
    }
}
