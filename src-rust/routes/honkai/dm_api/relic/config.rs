use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::BufReader,
};

use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

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

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
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
        "ExcelOutput/RelicConfig.json"
    }

    async fn upstream_convert(from: Self::TUpstream) -> Result<Self::TLocal, WorkerError> {
        let transformed = from
            .into_iter()
            .map(|(k, v)| {
                let value = RelicConfig {
                    id: v.id,
                    set_id: v.set_id,
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

impl RelicConfig {
    pub async fn write_splitted() -> Result<(), WorkerError> {
        let relic_db: HashMap<u32, RelicConfig> = RelicConfig::read().await?;
        let set_ids = relic_db
            .values()
            .map(|relic| relic.set_id)
            .collect::<HashSet<u32>>()
            .into_iter()
            .collect::<Vec<u32>>();

        std::fs::create_dir_all("/tmp/RelicConfigs")?;

        for set_id in set_ids.into_iter() {
            let relics = relic_db
                .values()
                .filter(|relic| relic.set_id == set_id)
                .cloned()
                .collect::<Vec<RelicConfig>>();
            let filepath = format!("/tmp/RelicConfigs/{}.json", set_id);
            let json_blob = serde_json::to_string(&relics)?;
            // save to a new file
            std::fs::write(filepath, json_blob)?;
        }
        Ok(())
    }

    pub async fn read_splitted_by_setid(set_id: u32) -> Result<Vec<RelicConfig>, WorkerError> {
        let filepath = format!("/tmp/RelicConfigs/{}.json", set_id);
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);
        let data: Vec<RelicConfig> = serde_json::from_reader(reader)?;
        Ok(data)
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, EnumIter, Hash, JsonSchema)]
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
