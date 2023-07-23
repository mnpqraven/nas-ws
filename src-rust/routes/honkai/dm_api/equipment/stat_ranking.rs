use super::{
    equipment_config::EquipmentConfig, equipment_promotion_config::EquipmentPromotionConfig,
};
use crate::{
    handler::error::WorkerError,
    routes::{endpoint_types::List, honkai::traits::DbData},
};
use axum::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct EquipmentRanking {
    pub equipment_id: u32,
    pub equipment_name: String,
    pub level: Vec<u32>, // max_level -1 * (add rate)
    pub hp: Vec<f64>,
    pub atk: Vec<f64>,
    pub def: Vec<f64>,
}
pub async fn stat_ranking() -> Result<Json<List<EquipmentRanking>>, WorkerError> {
    let promotion_db = EquipmentPromotionConfig::read().await?;
    let equipment_config = EquipmentConfig::read().await?;
    let equipment_config = Arc::new(equipment_config);

    let ranking: Vec<EquipmentRanking> = promotion_db
        .into_iter()
        .map(|(key, v)| {
            let cloned = v.max_level.clone();
            let clos = |list: Vec<f64>, add: Vec<f64>| {
                list.into_iter()
                    .enumerate()
                    .map(|(index, tier)| tier + (add[index] * (cloned[index] as f64 - 1.0)))
                    .collect()
            };
            let name = &equipment_config.get(&key).unwrap().equipment_name;

            EquipmentRanking {
                equipment_id: key,
                equipment_name: name.to_owned(),
                level: v.max_level,
                hp: clos(v.base_hp, v.base_hpadd),
                atk: clos(v.base_attack, v.base_attack_add),
                def: clos(v.base_defence, v.base_defence_add),
            }
        })
        .collect();
    Ok(Json(List::new(ranking)))
}
