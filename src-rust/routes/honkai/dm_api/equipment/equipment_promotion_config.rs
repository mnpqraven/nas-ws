use std::collections::{BTreeMap, HashMap};

use crate::{
    handler::error::WorkerError,
    routes::honkai::{
        dm_api::{character::types::MiniItem, types::Param},
        traits::DbData,
    },
};
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct UpstreamEquipmentPromotionConfig {
    #[serde(alias = "EquipmentID")]
    equipment_id: u32,
    #[serde(alias = "Promotion")]
    promotion: u32,
    #[serde(alias = "PromotionCostList")]
    promotion_cost_list: Vec<MiniItem>,
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
    pub promotion_cost_list: Vec<Vec<MiniItem>>,
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
impl DbData for EquipmentPromotionConfig {
    type TUpstream = HashMap<u32, BTreeMap<u32, UpstreamEquipmentPromotionConfig>>;
    type TLocal = HashMap<u32, EquipmentPromotionConfig>;

    fn path_data() -> &'static str {
        "ExcelOutput/EquipmentPromotionConfig.json"
    }

    async fn upstream_convert(
        from: HashMap<u32, BTreeMap<u32, UpstreamEquipmentPromotionConfig>>,
    ) -> Result<HashMap<u32, EquipmentPromotionConfig>, WorkerError> {
        let transformed = from
            .into_iter()
            .map(|(main_key, inner_map)| {
                let mut res = EquipmentPromotionConfig {
                    equipment_id: main_key,
                    ..Default::default()
                };
                for (_, value) in inner_map.into_iter() {
                    res.promotion.push(value.promotion);
                    res.promotion_cost_list.push(value.promotion_cost_list);
                    res.world_level_require
                        .push(value.world_level_require.unwrap_or_default());
                    res.max_level.push(value.max_level);
                    res.base_hp.push(value.base_hp.value);
                    res.base_hpadd.push(value.base_hpadd.value);
                    res.base_attack.push(value.base_attack.value);
                    res.base_attack_add.push(value.base_attack_add.value);
                    res.base_defence.push(value.base_defence.value);
                    res.base_defence_add.push(value.base_defence_add.value);
                }

                (main_key, res)
            })
            .collect();
        Ok(transformed)
    }
}
