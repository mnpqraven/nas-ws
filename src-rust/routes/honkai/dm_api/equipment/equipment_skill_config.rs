use crate::{
    builder::{get_db_client, traits::DbAction},
    handler::error::WorkerError,
    routes::honkai::{
        dm_api::{
            desc_param::{get_sorted_params, ParameterizedDescription},
            hash::TextHash,
            types::{AbilityProperty, Param, TextMap},
        },
        traits::DbData,
    },
};
use async_trait::async_trait;
use libsql_client::{args, Statement};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

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
    pub skill_id: u32,
    pub skill_name: String,
    pub skill_desc: ParameterizedDescription,
    pub level: Vec<u32>,
    pub ability_name: String,
    pub param_list: Vec<Vec<String>>,
    pub ability_property: Vec<Vec<AbilityProperty>>,
}

#[async_trait]
impl DbData for EquipmentSkillConfig {
    type TUpstream = HashMap<u32, BTreeMap<u32, UpstreamEquipmentSkillConfig>>;
    type TLocal = HashMap<u32, EquipmentSkillConfig>;

    fn path_data() -> &'static str {
        "ExcelOutput/EquipmentSkillConfig.json"
    }

    async fn upstream_convert(
        from: HashMap<u32, BTreeMap<u32, UpstreamEquipmentSkillConfig>>,
    ) -> Result<HashMap<u32, EquipmentSkillConfig>, WorkerError> {
        let text_map: HashMap<String, String> = TextMap::read().await?;

        let transformed: HashMap<u32, EquipmentSkillConfig> = from
            .iter()
            .map(|(key, inner_map)| {
                // NOTE: iterate through inner_map > sort (done via BTreeMap) > merge merge
                let first = inner_map.get(&1).unwrap(); // WARN: unwrap

                // multiple reads in `for_each`
                let skill_desc_raw = Arc::new(
                    text_map
                        .get(&first.skill_desc.hash.to_string())
                        .unwrap_or(&"NOT FOUND".into())
                        .to_string(),
                );

                let skill_desc: ParameterizedDescription = skill_desc_raw.to_string().into();

                let skill_name: String = text_map
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

                (*key, next)
            })
            .collect();
        Ok(transformed)
    }
}

#[async_trait]
impl DbAction for EquipmentSkillConfig {
    async fn seed() -> Result<(), WorkerError> {
        let client = get_db_client().await?;

        let lc_db = EquipmentSkillConfig::read().await?;
        let lc_sts = lc_db
            .into_values()
            .map(|v| {
                let EquipmentSkillConfig {
                    skill_id,
                    skill_name,
                    skill_desc,
                    param_list,
                    ability_property,
                    ..
                } = v;
                Statement::with_args(
                    "INSERT OR REPLACE INTO
                    honkai_lightConeSkill (
                        id, name, desc, param_list, ability_property
                    ) VALUES (?,?,?,?,?)",
                    args!(
                        skill_id,
                        skill_name,
                        serde_json::to_string(&skill_desc).unwrap(),
                        serde_json::to_string(&param_list).unwrap(),
                        serde_json::to_string(&ability_property).unwrap(),
                    ),
                )
            })
            .collect::<Vec<Statement>>();

        client.batch(lc_sts).await?;

        Ok(())
    }
    async fn teardown() -> Result<(), WorkerError> {
        let client = get_db_client().await?;
        client.execute("DELETE FROM honkai_lightConeSkill").await?;
        Ok(())
    }
}
