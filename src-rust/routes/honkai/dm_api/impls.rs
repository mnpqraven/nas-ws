use super::{
    constants::*,
    types::{
        EquipmentConfig, EquipmentConfigMerged, EquipmentSkillConfig, EquipmentSkillConfigMerged,
        TextMap,
    },
    BigTraceInfo,
};
use crate::routes::honkai::dm_api::DbData;
use crate::{handler::error::WorkerError, routes::honkai::mhy_api::internal::impls::DbDataLike};
use async_trait::async_trait;
use futures::StreamExt;
use regex::{Captures, Regex};
use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};
use tracing::{info, instrument};

impl<T: DbDataLike> DbData<T> for TextMap {
    fn path_data() -> (&'static str, &'static str) {
        (TEXT_MAP_LOCAL, TEXT_MAP_REMOTE)
    }
}

#[async_trait]
impl<T: DbDataLike> DbData<T> for EquipmentConfigMerged {
    fn path_data() -> (&'static str, &'static str) {
        (EQUIPMENT_CONFIG_LOCAL, EQUIPMENT_CONFIG_REMOTE)
    }

    async fn try_write_disk(local_path: &str) -> Result<String, WorkerError> {
        let (_, fallback_url) = <EquipmentConfigMerged as DbData<T>>::path_data();
        // EquipmentConfig
        let data = reqwest::get(fallback_url).await?.text().await?;
        let data: HashMap<String, EquipmentConfig> = serde_json::from_str(&data)?;

        // textmap chunk
        let text_map_chunk: HashMap<String, String> = TextMap::read().await?;
        // allow usage of generators (HashMap getter) inside `async move`
        // closures (which requires `Fn`)
        let arced_chunk = Arc::new(text_map_chunk);
        let to_write_db: HashMap<String, EquipmentConfigMerged> = data
            .iter()
            .map(|(key, value)| {
                let arced_chunk = arced_chunk.clone();
                let get_value = move |key: &str| arced_chunk.get(key).cloned().unwrap_or_default();

                let (eq_name, eq_desc) = (
                    get_value(&value.equipment_name.hash.to_string()),
                    get_value(&value.equipment_desc.hash.to_string()),
                );

                let value = value.to_merged((eq_name, eq_desc)).unwrap();
                (key.clone(), value)
            })
            .collect();

        let to_write_text = serde_json::to_string_pretty(&to_write_db)?;

        // convert to EquipmentSkillConfigMerged
        std::fs::write(local_path, &to_write_text)?;
        Ok(to_write_text)
    }
}

impl EquipmentConfig {
    fn to_merged(
        &self,
        (equipment_name, equipment_desc): (String, String),
    ) -> Result<EquipmentConfigMerged, WorkerError> {
        Ok(EquipmentConfigMerged {
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
        })
    }
}

#[async_trait]
impl<T: DbDataLike> DbData<T> for EquipmentSkillConfigMerged {
    fn path_data() -> (&'static str, &'static str) {
        (EQUIPMENT_SKILL_CONFIG_LOCAL, EQUIPMENT_SKILL_CONFIG_REMOTE)
    }

    // WARN: needs to traverse 1 depth and merge diffs, converting
    // Vec<Vec<EquipmentSkillConfig>> to this (serialized Vec<EquipmentSkillConfigMerged>)
    async fn try_write_disk(local_path: &str) -> Result<String, WorkerError> {
        let (_, fallback_url) = <EquipmentSkillConfigMerged as DbData<T>>::path_data();
        // EquipmentSkillConfig
        let data = reqwest::get(fallback_url).await?.text().await?;
        let data: HashMap<String, BTreeMap<String, EquipmentSkillConfig>> =
            serde_json::from_str(&data)?;

        // textmap chunk
        let text_map_chunk: HashMap<String, String> = TextMap::read().await?;

        // NOTE: probably better to fk hash here
        let to_write_db: HashMap<String, EquipmentSkillConfigMerged> = data
            .iter()
            .map(|(key, inner_map)| {
                // NOTE: iterate through inner_map > sort (done via BTreeMap) > merge merge
                let first = inner_map.get("1").unwrap(); // WARN: unwrap
                let skill_desc: String = text_map_chunk
                    .get(&first.skill_desc.hash.to_string())
                    .unwrap_or(&"NOT FOUND".into())
                    .to_owned();
                let skill_name: String = text_map_chunk
                    .get(&first.skill_name.hash.to_string())
                    .unwrap_or(&"NOT FOUND".into())
                    .to_owned();

                let mut next: EquipmentSkillConfigMerged = EquipmentSkillConfigMerged {
                    skill_id: first.skill_id,
                    skill_name,
                    skill_desc,
                    level: vec![],
                    ability_name: first.ability_name.clone(),
                    param_list: vec![],
                    ability_property: vec![],
                };

                inner_map.iter().for_each(|(_, skill_config)| {
                    next.level.push(skill_config.level);
                    next.param_list.push(skill_config.param_list.clone());
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

impl BigTraceInfo {
    const DESC_IDENT: &str = r"#\d\[.\d?\]%?";
    #[allow(dead_code)]
    #[instrument(ret)]
    pub fn parse_description(&self) -> Vec<String> {
        // desc
        // "Deals Lightning DMG equal to #1[i]% of Kafka's ATK to a single enemy.",
        // params
        // [ [0.5], [0.6] ,.. , [] ]
        let regex = Regex::new(Self::DESC_IDENT).unwrap();
        let mut res: Vec<String> = vec![];
        for param in self.params.iter() {
            let result = regex.replace_all(&self.desc, |caps: &Captures| {
                let mut res = String::new();
                for cap in caps.iter().flatten() {
                    let is_percent: bool = cap.as_str().ends_with('%');

                    // let index = cap.as_str().chars().nth(1).unwrap().to_digit(10).unwrap() as usize;

                    let params_data = match is_percent {
                        true => param * 100.0,
                        false => *param,
                    };
                    match is_percent {
                        true => res.push_str(&format!("{:.2}%", &params_data)),
                        false => res.push_str(&format!("{:.2}", &params_data)),
                    }
                }
                res
            });
            res.push(result.to_string());
        }
        info!("{:?}", res);
        res
    }

    pub fn split_description(&self) -> Arc<[Arc<str>]> {
        let regex = Regex::new(Self::DESC_IDENT).unwrap();
        let t: Arc<[Arc<str>]> = regex.split(&self.desc).map(|e| e.into()).collect();
        t
    }

    /// returns a tuple of
    /// 1. index of the params value
    /// 2. whether the params value should be displayed as percentage
    pub fn get_sorted_params_inds(&self) -> Vec<(usize, bool)> {
        let regex = Regex::new(Self::DESC_IDENT).unwrap();
        let inds = regex
            .find_iter(&self.desc)
            .map(|e| {
                let ind: usize = (e.as_str().chars().nth(1).unwrap().to_digit(10).unwrap() - 1)
                    .try_into()
                    .unwrap();
                let is_percent = e.as_str().ends_with('%');
                (ind, is_percent)
            })
            .collect::<Vec<(usize, bool)>>();
        inds
    }
}
