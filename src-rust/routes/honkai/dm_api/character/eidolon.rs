use super::upstream_avatar_config::MiniItem;
use crate::{
    handler::error::WorkerError,
    routes::honkai::{
        dm_api::{
            desc_param::{get_sorted_params, ParameterizedDescription},
            hash::{HashedString, TextHash},
            types::{Param, TextMap},
        },
        mhy_api::types_parsed::shared::AssetPath,
        traits::DbData,
    },
};
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpstreamAvatarRankConfig {
    #[serde(alias = "RankID")]
    rank_id: u32,
    #[serde(alias = "Rank")]
    rank: u32,
    #[serde(alias = "Trigger")]
    trigger: TextHash,
    #[serde(alias = "Name")]
    name: HashedString,
    #[serde(alias = "Desc")]
    desc: HashedString,
    #[serde(alias = "IconPath")]
    icon_path: AssetPath,
    #[serde(alias = "SkillAddLevelList")]
    skill_add_level_list: SkillAddLevelList,
    #[serde(alias = "RankAbility")]
    rank_ability: Vec<String>, // TODO: check if this is truly string or hashedString
    #[serde(alias = "UnlockCost")]
    unlock_cost: Vec<MiniItem>,
    #[serde(alias = "Param")]
    param: Vec<Param>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AvatarRankConfig {
    pub rank_id: u32,
    pub rank: u32,
    pub trigger: String,
    pub name: String,
    pub desc: ParameterizedDescription,
    pub icon_path: AssetPath,
    pub skill_add_level_list: SkillAddLevelList,
    pub rank_ability: Vec<String>, // TODO: check if this is truly string or hashedString
    pub unlock_cost: Vec<MiniItem>,
    pub param: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct SkillAddLevelList(HashMap<u32, u32>);

#[async_trait]
impl DbData for AvatarRankConfig {
    type TUpstream = HashMap<u32, UpstreamAvatarRankConfig>;
    type TLocal = HashMap<u32, AvatarRankConfig>;

    fn path_data() -> &'static str {
        "ExcelOutput/AvatarRankConfig.json"
    }
    async fn upstream_convert(
        from: HashMap<u32, UpstreamAvatarRankConfig>,
    ) -> Result<HashMap<u32, AvatarRankConfig>, WorkerError> {
        let text_map = TextMap::read().await?;
        let transformed = from
            .into_iter()
            .map(|(k, v)| {
                let unsplitted_desc = TextHash::from(v.desc.clone())
                    .read_from_textmap(&text_map)
                    .unwrap();
                let sorted_params: Vec<String> =
                    get_sorted_params(v.param.iter().map(|e| e.value).collect(), &unsplitted_desc)
                        .iter()
                        .map(|e| e.to_string())
                        .collect();
                let desc = v.desc.dehash(&text_map).unwrap_or_default();

                let data = AvatarRankConfig {
                    rank_id: v.rank_id,
                    rank: v.rank,
                    trigger: v.trigger.read_from_textmap(&text_map).unwrap_or_default(),
                    name: v.name.dehash(&text_map).unwrap_or_default(),
                    desc: desc.into(),
                    icon_path: v.icon_path,
                    skill_add_level_list: v.skill_add_level_list,
                    rank_ability: v.rank_ability, // TODO: check if this is truly string or hashedString
                    unlock_cost: v.unlock_cost,
                    param: sorted_params,
                };
                (k, data)
            })
            .collect();
        Ok(transformed)
    }
}
