use super::{
    constants::*,
    desc_param::{get_sorted_params, ParameterizedDescription},
    types::*,
};
use crate::routes::honkai::dm_api::DbData;
use crate::{handler::error::WorkerError, routes::honkai::mhy_api::internal::impls::DbDataLike};
use async_trait::async_trait;
use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

impl<T: DbDataLike> DbData<T> for TextMap {
    fn path_data() -> (&'static str, &'static str) {
        (TEXT_MAP_LOCAL, TEXT_MAP_REMOTE)
    }
}

#[async_trait]
impl<T: DbDataLike> DbData<T> for EquipmentConfig {
    fn path_data() -> (&'static str, &'static str) {
        (EQUIPMENT_CONFIG_LOCAL, EQUIPMENT_CONFIG_REMOTE)
    }

    async fn try_write_disk() -> Result<String, WorkerError> {
        let (local_path, fallback_url) = <EquipmentConfig as DbData<T>>::path_data();
        // EquipmentConfig
        let data = reqwest::get(fallback_url).await?.text().await?;
        let data: HashMap<String, UpstreamEquipmentConfig> = serde_json::from_str(&data)?;

        // textmap chunk
        let text_map_chunk: HashMap<String, String> = TextMap::read().await?;
        // allow usage of generators (HashMap getter) inside `async move`
        // closures (which requires `Fn`)
        let arced_chunk = Arc::new(text_map_chunk);
        let to_write_db: HashMap<String, EquipmentConfig> = data
            .iter()
            .map(|(key, value)| {
                let arced_chunk = arced_chunk.clone();
                let get_value = move |key: &str| arced_chunk.get(key).cloned().unwrap_or_default();

                let (eq_name, eq_desc) = (
                    get_value(&value.equipment_name.hash.to_string()),
                    get_value(&value.equipment_desc.hash.to_string()),
                );

                let value = value.to_merged((eq_name, eq_desc));
                (key.clone(), value)
            })
            .collect();

        let to_write_text = serde_json::to_string_pretty(&to_write_db)?;

        // convert to EquipmentSkillConfigMerged
        std::fs::write(local_path, &to_write_text)?;
        Ok(to_write_text)
    }
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
impl<T: DbDataLike> DbData<T> for EquipmentSkillConfig {
    fn path_data() -> (&'static str, &'static str) {
        (EQUIPMENT_SKILL_CONFIG_LOCAL, EQUIPMENT_SKILL_CONFIG_REMOTE)
    }

    // WARN: needs to traverse 1 depth and merge diffs, converting
    // Vec<Vec<UpstreamEquipmentSkillConfig>> to this (serialized Vec<EquipmentSkillConfig>)
    async fn try_write_disk() -> Result<String, WorkerError> {
        let (local_path, fallback_url) = <EquipmentSkillConfig as DbData<T>>::path_data();
        // EquipmentSkillConfig
        let data = reqwest::get(fallback_url).await?.text().await?;
        let data: HashMap<String, BTreeMap<String, UpstreamEquipmentSkillConfig>> =
            serde_json::from_str(&data)?;

        // textmap chunk
        let text_map_chunk: HashMap<String, String> = TextMap::read().await?;

        // NOTE: probably better to fk hash here
        let to_write_db: HashMap<String, EquipmentSkillConfig> = data
            .iter()
            .map(|(key, inner_map)| {
                // NOTE: iterate through inner_map > sort (done via BTreeMap) > merge merge
                let first = inner_map.get("1").unwrap(); // WARN: unwrap

                // multiple reads in `for_each`
                let skill_desc_raw = Arc::new(
                    text_map_chunk
                        .get(&first.skill_desc.hash.to_string())
                        .unwrap_or(&"NOT FOUND".into())
                        .to_string(),
                );

                let skill_desc: ParameterizedDescription = skill_desc_raw.to_string().into();

                let skill_name: String = text_map_chunk
                    .get(&first.skill_name.hash.to_string())
                    .unwrap_or(&"NOT FOUND".into())
                    .to_owned();

                let mut next: EquipmentSkillConfig = EquipmentSkillConfig {
                    skill_id: first.skill_id,
                    skill_name,
                    skill_desc,
                    level: vec![],
                    ability_name: first.ability_name.clone(),
                    param_list: vec![],
                    ability_property: vec![],
                };

                inner_map.iter().for_each(|(_key, skill_config)| {
                    next.level.push(skill_config.level);
                    let sorted_params: Vec<String> = get_sorted_params(
                        skill_config
                            .param_list
                            .iter()
                            .map(|param| param.value)
                            .collect::<Vec<f64>>(),
                        &skill_desc_raw,
                    )
                    .iter()
                    .map(|e| e.to_string())
                    .collect();
                    next.param_list.push(sorted_params);
                    next.ability_property
                        .push(skill_config.ability_property.clone());
                });

                (key.clone(), next)
            })
            .collect();

        let to_write_text = serde_json::to_string_pretty(&to_write_db)?;

        // convert to EquipmentSkillConfigMerged
        std::fs::write(local_path, &to_write_text)?;

        Ok(to_write_text)
    }
}
