use super::config::RelicType;
use crate::{
    handler::error::WorkerError,
    routes::honkai::{
        dm_api::types::{Param, Property},
        traits::DbData,
    },
};
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use strum::IntoEnumIterator;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpstreamRelicMainAffixConfig {
    #[serde(alias = "GroupID")]
    group_id: u32,
    #[serde(alias = "AffixID")]
    affix_id: u32,
    #[serde(alias = "Property")]
    property: Property,
    #[serde(alias = "BaseValue")]
    base_value: Param,
    #[serde(alias = "LevelAdd")]
    level_add: Param,
    #[serde(alias = "IsAvailable")]
    is_available: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct RelicMainAffixConfig {
    #[serde(alias = "GroupID")]
    group_id: u32,
    #[serde(alias = "AffixID")]
    affix_id: u32,
    #[serde(alias = "Property")]
    property: Property,
    #[serde(alias = "BaseValue")]
    base_value: f64,
    #[serde(alias = "LevelAdd")]
    level_add: f64,
    #[serde(alias = "IsAvailable")]
    is_available: bool,
}

impl From<UpstreamRelicMainAffixConfig> for RelicMainAffixConfig {
    fn from(value: UpstreamRelicMainAffixConfig) -> Self {
        let UpstreamRelicMainAffixConfig {
            group_id,
            affix_id,
            property,
            base_value,
            level_add,
            is_available,
        } = value;
        Self {
            group_id,
            affix_id,
            property,
            base_value: base_value.value,
            level_add: level_add.value,
            is_available,
        }
    }
}

#[async_trait]
impl DbData for RelicMainAffixConfig {
    // first BTree's key is for rarity
    // second BTree's key is normal indexing
    type TUpstream = BTreeMap<u32, BTreeMap<u32, UpstreamRelicMainAffixConfig>>;
    // for now ignore lower rarities, only take 5
    type TLocal = BTreeMap<RelicType, Vec<RelicMainAffixConfig>>;

    fn path_data() -> &'static str {
        "ExcelOutput/RelicMainAffixConfig.json"
    }

    async fn upstream_convert(from: Self::TUpstream) -> Result<Self::TLocal, WorkerError> {
        let mut transformed = BTreeMap::new();
        for relic_type in RelicType::iter() {
            let key = match relic_type {
                RelicType::HEAD => 51,
                RelicType::HAND => 52,
                RelicType::BODY => 53,
                RelicType::FOOT => 54,
                RelicType::OBJECT => 55,
                RelicType::NECK => 56,
            };
            if let Some(find) = from.get(&key) {
                transformed.insert(
                    relic_type,
                    find.clone().into_values().map(|val| val.into()).collect(),
                );
            } else {
                return Err(WorkerError::NotFound(format!(
                    "mainStatConfig with groupId {}",
                    key
                )));
            }
        }
        Ok(transformed)
    }
}
