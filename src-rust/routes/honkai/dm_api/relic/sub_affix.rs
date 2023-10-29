use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::{
    handler::error::WorkerError,
    routes::honkai::{
        dm_api::types::{Param, Property},
        traits::DbData,
    },
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpstreamRelicSubAffixConfig {
    #[serde(alias = "GroupID")]
    group_id: u32,
    #[serde(alias = "AffixID")]
    affix_id: u32,
    #[serde(alias = "Property")]
    property: Property,
    #[serde(alias = "BaseValue")]
    base_value: Param,
    #[serde(alias = "StepValue")]
    step_value: Param,
    #[serde(alias = "StepNum")]
    step_num: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct RelicSubAffixConfig {
    group_id: u32,
    affix_id: u32,
    property: Property,
    base_value: f64,
    step_value: f64,
    step_num: u32,
}

impl From<UpstreamRelicSubAffixConfig> for RelicSubAffixConfig {
    fn from(value: UpstreamRelicSubAffixConfig) -> Self {
        let UpstreamRelicSubAffixConfig {
            group_id,
            affix_id,
            property,
            base_value,
            step_value,
            step_num,
        } = value;
        Self {
            group_id,
            affix_id,
            property,
            base_value: base_value.value,
            step_value: step_value.value,
            step_num,
        }
    }
}

#[async_trait]
impl DbData for RelicSubAffixConfig {
    // first BTree's key is for rarity
    // second BTree's key is normal indexing
    type TUpstream = BTreeMap<u32, BTreeMap<u32, UpstreamRelicSubAffixConfig>>;
    // for now ignore lower rarities, only take 5
    type TLocal = BTreeMap<u32, RelicSubAffixConfig>;

    fn path_data() -> &'static str {
        "ExcelOutput/RelicSubAffixConfig.json"
    }

    async fn upstream_convert(from: Self::TUpstream) -> Result<Self::TLocal, WorkerError> {
        match from.get(&5) {
            Some(value) => {
                let transformed = value
                    .iter()
                    .map(|(key, value)| (*key, value.clone().into()))
                    .collect();
                Ok(transformed)
            }
            _ => Err(WorkerError::NotFound(
                "5* data for said SubAffixConfig".to_string(),
            )),
        }
    }
}
