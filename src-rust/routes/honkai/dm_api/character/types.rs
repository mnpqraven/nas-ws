use crate::{
    builder::{get_db_client, traits::DbAction, AsyncInto},
    handler::error::WorkerError,
    routes::honkai::{
        dm_api::{
            hash::TextHash,
            types::{AssetPath, Element, Param, Path, TextMap},
        },
        traits::DbData,
    },
};
use async_trait::async_trait;
use libsql_client::{args, Statement};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub reward_list: Vec<MiniItem>,
    #[serde(alias = "RewardListMax")]
    pub reward_list_max: Vec<MiniItem>,
    #[serde(alias = "SkillList")]
    pub skill_list: Vec<u32>,
    #[serde(alias = "AvatarBaseType")]
    pub avatar_base_type: Path,
    #[serde(alias = "AvatarDesc")]
    pub avatar_desc: TextHash,
    #[serde(alias = "DamageTypeResistance")]
    pub damage_type_resistance: Vec<DamageTypeResistance>,
    #[serde(alias = "Release")]
    pub release: Option<bool>,
    #[serde(alias = "AvatarCutinIntroText")]
    pub avatar_cutin_intro_text: TextHash,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AvatarConfig {
    pub avatar_id: u32,
    pub avatar_name: String,
    #[serde(skip)]
    avatar_full_name: String,
    #[serde(skip)]
    adventure_player_id: u32,
    pub avatar_votag: String,
    pub rarity: u8,
    #[serde(skip)]
    json_path: AssetPath,
    pub damage_type: Element,
    pub spneed: u32,
    #[serde(skip)]
    exp_group: u32,
    #[serde(skip)]
    max_promotion: u8,
    #[serde(skip)]
    max_rank: u8,
    pub rank_idlist: Vec<u32>,
    #[serde(skip)]
    reward_list: Vec<MiniItem>,
    #[serde(skip)]
    reward_list_max: Vec<MiniItem>,
    pub skill_list: Vec<u32>,
    pub avatar_base_type: Path,
    pub avatar_desc: String,
    damage_type_resistance: Vec<DamageTypeResistance>,
    pub release: bool,
    #[serde(skip)]
    avatar_cutin_intro_text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AvatarRarity {
    CombatPowerAvatarRarityType4 = 4,
    CombatPowerAvatarRarityType5 = 5,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename(serialize = "camelCase"))]
pub struct MiniItem {
    #[serde(alias = "ItemID")]
    pub item_id: u32,
    #[serde(alias = "ItemNum")]
    pub item_num: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct DamageTypeResistance {
    #[serde(alias = "DamageType")]
    damage_type: Element,
    #[serde(alias = "Value")]
    value: Param,
}

#[async_trait]
impl DbData for AvatarConfig {
    type TUpstream = HashMap<u32, UpstreamAvatarConfig>;
    type TLocal = HashMap<u32, AvatarConfig>;

    fn path_data() -> &'static str {
        "ExcelOutput/AvatarConfig.json"
    }

    async fn upstream_convert(
        from: HashMap<u32, UpstreamAvatarConfig>,
    ) -> Result<HashMap<u32, AvatarConfig>, WorkerError> {
        let text_map: HashMap<String, String> = TextMap::read().await?;
        let data = from
            .into_iter()
            .map(|(k, v)| {
                let v = v.into_using_resource(&text_map).unwrap();
                (k, v)
            })
            .collect();
        Ok(data)
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
            spneed: spneed.value as u32,
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
            release: release.unwrap_or(false),
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
        let name = avatar_name.read_from_textmap(text_map)?;
        let sanitized_tb_name = if name.eq("{NICKNAME}") {
            format!("Trailblazer ({})", damage_type)
        } else {
            name
        };
        let res = AvatarConfig {
            avatar_id,
            avatar_name: sanitized_tb_name,
            avatar_full_name: avatar_full_name.read_from_textmap(text_map)?,
            adventure_player_id,
            avatar_votag,
            rarity: rarity as u8,
            json_path,
            damage_type,
            spneed: spneed.value as u32,
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
            release: release.unwrap_or(false),
            avatar_cutin_intro_text: avatar_cutin_intro_text.read_from_textmap(text_map)?,
        };

        Ok(res)
    }
}

#[async_trait]
impl DbAction for AvatarConfig {
    async fn seed() -> Result<(), WorkerError> {
        let client = get_db_client().await?;
        let avatar_db = AvatarConfig::read().await?;
        let batch_avatar: Vec<Statement> = avatar_db
            .into_values()
            .map(
                |AvatarConfig {
                     avatar_id,
                     avatar_name,
                     avatar_votag,
                     rarity,
                     damage_type,
                     avatar_base_type,
                     spneed,
                     ..
                 }| {
                    Statement::with_args(
                        "INSERT OR REPLACE INTO honkai_avatar VALUES (?, ?, ?, ?, ?, ?, ?)",
                        args!(
                            avatar_id,
                            avatar_name,
                            rarity,
                            avatar_votag,
                            damage_type.to_string(),
                            avatar_base_type.to_string(),
                            spneed
                        ),
                    )
                },
            )
            .collect();

        client.batch(batch_avatar).await?;
        Ok(())
    }

    async fn teardown() -> Result<(), WorkerError> {
        let client = get_db_client().await?;
        client.execute("DELETE FROM honkai_avatar").await?;
        Ok(())
    }
}
