use crate::{
    handler::error::WorkerError,
    routes::honkai::{
        dm_api::{
            desc_param::{get_sorted_params, ParameterizedDescription},
            hash::HashedString,
            types::{Param, Property, TextMap},
        },
        traits::DbData,
    },
};
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpStreamRelicSetSkillConfig {
    #[serde(alias = "SetID")]
    set_id: u32,
    #[serde(alias = "RequireNum")]
    require_num: u8,
    #[serde(alias = "SkillDesc")]
    skill_desc: HashedString,
    #[serde(alias = "PropertyList")]
    property_list: Vec<HashMap<String, serde_json::Value>>,
    #[serde(alias = "AbilityName")]
    ability_name: HashedString,
    #[serde(alias = "AbilityParamList")]
    ability_param_list: Vec<Param>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct RelicParam {
    pub property: Property,
    pub value: f64,
}
impl Default for RelicParam {
    fn default() -> Self {
        Self {
            property: Property::Attack,
            value: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct RelicSetSkillConfig {
    pub set_id: u32,
    pub require_num: Vec<u8>,
    pub skill_desc: Vec<ParameterizedDescription>,
    pub property_list: Vec<Vec<RelicParam>>,
    pub ability_name: Vec<ParameterizedDescription>,
    pub ability_param_list: Vec<Vec<String>>,
}

#[async_trait]
impl DbData for RelicSetSkillConfig {
    type TUpstream = HashMap<u32, BTreeMap<u8, UpStreamRelicSetSkillConfig>>;
    type TLocal = HashMap<u32, RelicSetSkillConfig>;

    fn path_data() -> &'static str {
        "ExcelOutput/RelicSetSkillConfig.json"
    }

    async fn upstream_convert(
        from: HashMap<u32, BTreeMap<u8, UpStreamRelicSetSkillConfig>>,
    ) -> Result<HashMap<u32, RelicSetSkillConfig>, WorkerError> {
        let text_map = TextMap::read().await?;

        let transformed = from
            .into_iter()
            .map(|(key, v)| {
                let hashed_desc: Vec<HashedString> =
                    v.values().map(|e| e.skill_desc.clone()).collect();
                let dehashed_desc: Vec<ParameterizedDescription> = hashed_desc
                    .iter()
                    .map(|hashed| {
                        let dehashed = hashed.dehash(&text_map).unwrap_or_default();
                        ParameterizedDescription::from(dehashed)
                    })
                    .collect();

                let param_as_string: Vec<Vec<String>> = v
                    .values()
                    .map(|cfg| {
                        let params = cfg.ability_param_list.iter().map(|e| e.value).collect();
                        let desc_dehashed = cfg.skill_desc.dehash(&text_map).unwrap_or_default();
                        let current_param: Vec<String> = get_sorted_params(params, &desc_dehashed)
                            .iter()
                            .map(|e| e.to_string())
                            .collect();
                        current_param
                    })
                    .collect();

                let value = RelicSetSkillConfig {
                    set_id: key,
                    require_num: v.values().map(|e| e.require_num).collect(),
                    skill_desc: dehashed_desc,
                    // property_list: v.values().map(|e| e.property_list.clone()).collect(),
                    property_list: v
                        .values()
                        .map(|e| {
                            let mut ret: Vec<RelicParam> = Vec::new();
                            for prop in &e.property_list {
                                let mut relic = RelicParam::default();
                                for val in prop.values() {
                                    match val.is_string() {
                                        true => {
                                            relic.property =
                                                Property::from_str(val.as_str().unwrap()).unwrap()
                                        }
                                        false => relic.value = val["Value"].as_f64().unwrap(),
                                    }
                                }
                                ret.push(relic)
                            }
                            ret
                        })
                        .collect(),
                    ability_name: vec![],
                    ability_param_list: param_as_string,
                };
                (key, value)
            })
            .collect();

        Ok(transformed)
    }
}
