use crate::{
    handler::error::WorkerError,
    routes::{endpoint_types::List, honkai::traits::DbData},
};
use axum::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::equipment_promotion_config::EquipmentPromotionConfig;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct EquipmentRanking {
    pub equipment_id: u32,
    pub level: Vec<u32>, // max_level -1 * (add rate)
    pub hp: Vec<f64>,
    pub atk: Vec<f64>,
    pub def: Vec<f64>,
}
pub async fn stat_ranking() -> Result<Json<List<EquipmentRanking>>, WorkerError> {
    let promotion_db: HashMap<String, EquipmentPromotionConfig> =
        EquipmentPromotionConfig::read().await?;

    let ranking: Vec<EquipmentRanking> = promotion_db
        .into_iter()
        .map(|(k, v)| {
            let cloned = v.max_level.clone();
            let equipment_id = k.parse().unwrap();
            let clos = |list: Vec<f64>, add: Vec<f64>| {
                list.into_iter()
                    .enumerate()
                    .map(|(index, tier)| tier + (add[index] * (cloned[index] as f64 - 1.0)))
                    .collect()
            };
            EquipmentRanking {
                equipment_id,
                level: v.max_level,
                hp: clos(v.base_hp, v.base_hpadd),
                atk: clos(v.base_attack, v.base_attack_add),
                def: clos(v.base_defence, v.base_defence_add),
            }
        })
        .collect();
    Ok(Json(List::new(ranking)))
}
