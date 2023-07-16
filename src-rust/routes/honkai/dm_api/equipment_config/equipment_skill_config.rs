use crate::{
    handler::error::WorkerError,
    routes::honkai::{
        dm_api::{
            desc_param::{get_sorted_params, ParameterizedDescription},
            hash::TextHash,
            types::{AbilityProperty, Param, TextMap},
        },
        traits::{DbData, DbDataLike},
    },
};
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

#[cfg(target_os = "windows")]
pub const EQUIPMENT_SKILL_CONFIG_LOCAL: &str = "c:\\tmp\\equipment_skill_config.json";
#[cfg(target_os = "linux")]
pub const EQUIPMENT_SKILL_CONFIG_LOCAL: &str = "/tmp/equipment_skill_config.json";

pub const EQUIPMENT_SKILL_CONFIG_REMOTE: &str =
    "https://raw.githubusercontent.com/Dimbreath/StarRailData/master/ExcelOutput/EquipmentSkillConfig.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct UpstreamEquipmentSkillConfig {
    #[serde(alias = "SkillID")]
    pub skill_id: u32,
    #[serde(alias = "SkillName")]
    pub skill_name: TextHash,
    #[serde(alias = "SkillDesc")]
    pub skill_desc: TextHash,
    #[serde(alias = "Level")]
    pub level: u32,
    #[serde(alias = "AbilityName")]
    pub ability_name: String,
    #[serde(alias = "ParamList")]
    pub param_list: Vec<Param>,
    #[serde(alias = "AbilityProperty")]
    pub ability_property: Vec<AbilityProperty>,
}

/// skill info for light cones
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct EquipmentSkillConfig {
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
