use std::collections::HashMap;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{handler::error::WorkerError, routes::honkai::traits::DbData};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpstreamRelicConfig {
    #[serde(alias = "ID")]
    id: u32,
    #[serde(alias = "SetID")]
    set_id: u32,
    #[serde(alias = "Type")]
    ttype: RelicType,
    #[serde(alias = "Rarity")]
    rarity: RelicRarity,
    #[serde(alias = "MainAffixGroup")]
    main_affix_group: u32,
    #[serde(alias = "SubAffixGroup")]
    sub_affix_group: u32,
    #[serde(alias = "MaxLevel")]
    max_level: u32,
    #[serde(alias = "ExpType")]
    exp_type: u32,
    #[serde(alias = "ExpProvide")]
    exp_provide: u32,
    #[serde(alias = "CoinCost")]
    coin_cost: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RelicConfig {
    pub id: u32,
    pub set_id: u32,
    pub ttype: RelicType,
    pub rarity: u8,
    pub main_affix_group: u32,
    pub sub_affix_group: u32,
    pub max_level: u32,
    pub exp_type: u32,
    pub exp_provide: u32,
    pub coin_cost: u32,
}

#[async_trait]
impl DbData for RelicConfig {
    type TUpstream = HashMap<u32, UpstreamRelicConfig>;
    type TLocal = HashMap<u32, RelicConfig>;

    fn path_data() -> &'static str {
        "ExcelConfig/RelicConfig.json"
    }

    async fn upstream_convert(from: Self::TUpstream) -> Result<Self::TLocal, WorkerError> {
        let transformed = from
            .into_iter()
            .map(|(k, v)| {
                let value = RelicConfig {
                    id: v.id,
                    set_id: v.id,
                    rarity: v.rarity as u8,
                    ttype: v.ttype,
                    exp_type: v.exp_type,
                    max_level: v.max_level,
                    coin_cost: v.coin_cost,
                    exp_provide: v.exp_provide,
                    sub_affix_group: v.sub_affix_group,
                    main_affix_group: v.main_affix_group,
                };
                (k, value)
            })
            .collect();
        Ok(transformed)
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RelicType {
    HEAD,
    HAND,
    BODY,
    FOOT,
    OBJECT,
    NECK,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RelicRarity {
    CombatPowerRelicRarity2 = 2,
    CombatPowerRelicRarity3 = 3,
    CombatPowerRelicRarity4 = 4,
    CombatPowerRelicRarity5 = 5,
}
