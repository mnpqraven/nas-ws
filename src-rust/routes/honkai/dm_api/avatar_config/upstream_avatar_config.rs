use std::collections::HashMap;

use crate::{
    builder::AsyncInto,
    handler::error::WorkerError,
    routes::honkai::{
        dm_api::{hash::TextHash, types::Param},
        mhy_api::{
            internal::impls::{DbData, DbDataLike},
            types_parsed::shared::{AssetPath, Element, Path},
        },
    },
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[cfg(target_os = "windows")]
const AVATAR_CONFIG_LOCAL: &str = "c:\\tmp\\avatar_config.json";
#[cfg(target_os = "linux")]
const AVATAR_CONFIG_LOCAL: &str = "/tmp/avatar_config.json";

const AVATAR_CONFIG_REMOTE: &str =
    "https://raw.githubusercontent.com/Dimbreath/StarRailData/master/ExcelOutput/AvatarConfig.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpstreamAvatarConfig {
    #[serde(alias = "AvatarID")]
    pub avatar_id: u32,
    #[serde(alias = "AvatarName")]
    pub avatar_name: TextHash,
    #[serde(alias = "AvatarFullName")]
    pub avatar_full_name: TextHash,
    #[serde(alias = "AdventurePlayerID")]
    pub adventure_player_id: u32,
    #[serde(alias = "AvatarVOTag")]
    pub avatar_votag: String,
    #[serde(alias = "Rarity")]
    pub rarity: AvatarRarity,
    #[serde(alias = "JsonPath")]
    pub json_path: AssetPath,
    #[serde(alias = "DamageType")]
    pub damage_type: Element,
    #[serde(alias = "SPNeed")]
    pub spneed: Param,
    #[serde(alias = "ExpGroup")]
    pub exp_group: u32,
    #[serde(alias = "MaxPromotion")]
    pub max_promotion: u8,
    #[serde(alias = "MaxRank")]
    pub max_rank: u8,
    #[serde(alias = "RankIDList")]
    pub rank_idlist: Vec<u32>,
    #[serde(alias = "RewardList")]
    pub reward_list: Vec<Item>,
    #[serde(alias = "RewardListMax")]
    pub reward_list_max: Vec<Item>,
    #[serde(alias = "SkillList")]
    pub skill_list: Vec<u32>,
    #[serde(alias = "AvatarBaseType")]
    pub avatar_base_type: Path,
    #[serde(alias = "AvatarDesc")]
    pub avatar_desc: TextHash,
    #[serde(alias = "DamageTypeResistance")]
    pub damage_type_resistance: Vec<DamageTypeResistance>,
    #[serde(alias = "Release")]
    pub release: bool,
    #[serde(alias = "AvatarCutinIntroText")]
    pub avatar_cutin_intro_text: TextHash,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename(serialize = "camelCase"))]
pub struct AvatarConfig {
    #[serde(alias = "AvatarID")]
    pub avatar_id: u32,
    #[serde(alias = "AvatarName")]
    pub avatar_name: String,
    #[serde(alias = "AvatarFullName")]
    #[serde(skip)]
    avatar_full_name: String,
    #[serde(alias = "AdventurePlayerID")]
    #[serde(skip)]
    adventure_player_id: u32,
    #[serde(alias = "AvatarVOTag")]
    pub avatar_votag: String,
    #[serde(alias = "Rarity")]
    pub rarity: u8,
    #[serde(alias = "JsonPath")]
    #[serde(skip)]
    json_path: AssetPath,
    #[serde(alias = "DamageType")]
    pub damage_type: Element,
    #[serde(alias = "SPNeed")]
    pub spneed: f64,
    #[serde(alias = "ExpGroup")]
    #[serde(skip)]
    exp_group: u32,
    #[serde(alias = "MaxPromotion")]
    #[serde(skip)]
    max_promotion: u8,
    #[serde(alias = "MaxRank")]
    #[serde(skip)]
    max_rank: u8,
    #[serde(alias = "RankIDList")]
    pub rank_idlist: Vec<u32>,
    #[serde(alias = "RewardList")]
    #[serde(skip)]
    reward_list: Vec<Item>,
    #[serde(alias = "RewardListMax")]
    #[serde(skip)]
    reward_list_max: Vec<Item>,
    #[serde(alias = "SkillList")]
    pub skill_list: Vec<u32>,
    #[serde(alias = "AvatarBaseType")]
    pub avatar_base_type: Path,
    #[serde(alias = "AvatarDesc")]
    pub avatar_desc: String,
    #[serde(alias = "DamageTypeResistance")]
    damage_type_resistance: Vec<DamageTypeResistance>,
    #[serde(alias = "Release")]
    pub release: bool,
    #[serde(alias = "AvatarCutinIntroText")]
    #[serde(skip)]
    avatar_cutin_intro_text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AvatarRarity {
    CombatPowerAvatarRarityType4 = 4,
    CombatPowerAvatarRarityType5 = 5,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    #[serde(alias = "ItemID")]
    item_id: u32,
    #[serde(alias = "ItemNum")]
    item_num: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DamageTypeResistance {
    #[serde(alias = "DamageType")]
    damage_type: Element,
    #[serde(alias = "Value")]
    value: Param,
}

impl<T: DbDataLike> DbData<T> for UpstreamAvatarConfig {
    fn path_data() -> (&'static str, &'static str) {
        (AVATAR_CONFIG_LOCAL, AVATAR_CONFIG_REMOTE)
    }
}

#[async_trait]
impl AsyncInto<AvatarConfig> for UpstreamAvatarConfig {
    type Resource = HashMap<String, String>;

    async fn async_into(self) -> Result<AvatarConfig, WorkerError> {
        let UpstreamAvatarConfig {
            avatar_id,
            avatar_name,
            avatar_full_name,
            adventure_player_id,
            avatar_votag,
            rarity,
            json_path,
            damage_type,
            spneed,
            exp_group,
            max_promotion,
            max_rank,
            rank_idlist,
            reward_list,
            reward_list_max,
            skill_list,
            avatar_base_type,
            avatar_desc,
            damage_type_resistance,
            release,
            avatar_cutin_intro_text,
        } = self;
        let res = AvatarConfig {
            avatar_id,
            avatar_name: avatar_name.async_read_from_textmap().await?,
            avatar_full_name: avatar_full_name.async_read_from_textmap().await?,
            adventure_player_id,
            avatar_votag,
            rarity: rarity as u8,
            json_path,
            damage_type,
            spneed: spneed.into(),
            exp_group,
            max_promotion,
            max_rank,
            rank_idlist,
            reward_list,
            reward_list_max,
            skill_list,
            avatar_base_type,
            avatar_desc: avatar_desc.async_read_from_textmap().await?,
            damage_type_resistance,
            release,
            avatar_cutin_intro_text: avatar_cutin_intro_text.async_read_from_textmap().await?,
        };
        Ok(res)
    }

    fn into_using_resource(
        self,
        text_map: &HashMap<String, String>,
    ) -> Result<AvatarConfig, WorkerError> {
        let UpstreamAvatarConfig {
            avatar_id,
            avatar_name,
            avatar_full_name,
            adventure_player_id,
            avatar_votag,
            rarity,
            json_path,
            damage_type,
            spneed,
            exp_group,
            max_promotion,
            max_rank,
            rank_idlist,
            reward_list,
            reward_list_max,
            skill_list,
            avatar_base_type,
            avatar_desc,
            damage_type_resistance,
            release,
            avatar_cutin_intro_text,
        } = self;
        let res = AvatarConfig {
            avatar_id,
            avatar_name: avatar_name.read_from_textmap(text_map)?,
            avatar_full_name: avatar_full_name.read_from_textmap(text_map)?,
            adventure_player_id,
            avatar_votag,
            rarity: rarity as u8,
            json_path,
            damage_type,
            spneed: spneed.into(), // TODO: doublecheck
            exp_group,
            max_promotion,
            max_rank,
            rank_idlist,
            reward_list,
            reward_list_max,
            skill_list,
            avatar_base_type,
            avatar_desc: avatar_desc.read_from_textmap(text_map)?,
            damage_type_resistance,
            release,
            avatar_cutin_intro_text: avatar_cutin_intro_text.read_from_textmap(text_map)?,
        };

        Ok(res)
    }
}
