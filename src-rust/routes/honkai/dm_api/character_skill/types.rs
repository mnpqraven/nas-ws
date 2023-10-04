use crate::{
    handler::error::WorkerError,
    routes::honkai::{
        dm_api::{
            character::upstream_avatar_config::MiniItem,
            desc_param::{get_sorted_params, ParameterizedDescription},
            hash::{HashedString, TextHash},
            types::{AbilityProperty, Param, TextMap},
        },
        mhy_api::{
            internal::categorizing::{Anchor, SkillType},
            types_parsed::shared::{AssetPath, Element},
        },
        traits::DbData,
    },
};
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::BufReader,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpstreamAvatarSkillConfig {
    #[serde(alias = "SkillID")]
    skill_id: u32,
    #[serde(alias = "SkillName")]
    skill_name: TextHash,
    #[serde(alias = "SkillTag")]
    skill_tag: TextHash,
    #[serde(alias = "SkillTypeDesc")]
    skill_type_desc: TextHash,
    #[serde(alias = "Level")]
    level: u32,
    #[serde(alias = "MaxLevel")]
    max_level: u32,
    #[serde(alias = "SkillTriggerKey")]
    skill_trigger_key: String,
    #[serde(alias = "SkillIcon")]
    skill_icon: AssetPath,
    #[serde(alias = "UltraSkillIcon")]
    ultra_skill_icon: AssetPath,
    // WARN: unknown type, DM data is all empty
    #[serde(alias = "LevelUpCostList")]
    level_up_cost_list: Vec<u32>,
    #[serde(alias = "SkillDesc")]
    skill_desc: TextHash,
    #[serde(alias = "SimpleSkillDesc")]
    simple_skill_desc: TextHash,
    #[serde(alias = "RatedSkillTreeID")]
    rated_skill_tree_id: Vec<u32>,
    #[serde(alias = "RatedRankID")]
    rated_rank_id: Vec<u32>,
    // WARN: unknown type, DM data is all empty
    #[serde(alias = "ExtraEffectIDList")]
    extra_effect_idlist: Vec<u32>,
    // WARN: unknown type, DM data is all empty
    #[serde(alias = "SimpleExtraEffectIDList")]
    simple_extra_effect_idlist: Vec<u32>,
    #[serde(alias = "ShowStanceList")]
    show_stance_list: Vec<Param>,
    // WARN: unknown type, DM data is all empty
    #[serde(alias = "ShowDamageList")]
    // WARN: unknown type, DM data is all empty
    show_damage_list: Vec<u32>,
    #[serde(alias = "ShowHealList")]
    show_heal_list: Vec<u32>,
    #[serde(alias = "InitCoolDown")]
    init_cool_down: i32,
    #[serde(alias = "CoolDown")]
    cool_down: i32,
    #[serde(alias = "SPBase")]
    spbase: Option<Param>,
    #[serde(alias = "SPNeed")]
    spneed: Option<Param>,
    #[serde(alias = "SPMultipleRatio")]
    spmultiple_ratio: Param,
    #[serde(alias = "BPNeed")]
    bpneed: Option<Param>,
    #[serde(alias = "BPAdd")]
    bpadd: Option<Param>,
    #[serde(alias = "SkillNeed")]
    skill_need: TextHash,
    #[serde(alias = "DelayRatio")]
    delay_ratio: Param,
    #[serde(alias = "ParamList")]
    param_list: Vec<Param>,
    #[serde(alias = "SimpleParamList")]
    simple_param_list: Vec<Param>,
    #[serde(alias = "StanceDamageType")]
    stance_damage_type: Option<Element>,
    #[serde(alias = "AttackType")]
    attack_type: Option<SkillType>,
    #[serde(alias = "SkillEffect")]
    skill_effect: SKillEffect,
    #[serde(alias = "SkillComboValueDelta")]
    skill_combo_value_delta: Option<Param>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AvatarSkillConfig {
    #[serde(alias = "SkillID")]
    pub skill_id: u32,
    #[serde(alias = "SkillName")]
    skill_name: String,
    #[serde(alias = "SkillTag")]
    skill_tag: String,
    #[serde(alias = "SkillTypeDesc")]
    skill_type_desc: String,
    #[serde(alias = "Level")]
    level: Vec<u32>,
    #[serde(alias = "MaxLevel")]
    max_level: u32,
    #[serde(alias = "SkillTriggerKey")]
    skill_trigger_key: String,
    #[serde(alias = "SkillIcon")]
    skill_icon: AssetPath,
    #[serde(alias = "UltraSkillIcon")]
    ultra_skill_icon: AssetPath,
    // WARN: unknown type, DM data is all empty
    #[serde(alias = "LevelUpCostList")]
    level_up_cost_list: Vec<u32>,
    #[serde(alias = "SkillDesc")]
    skill_desc: ParameterizedDescription,
    #[serde(alias = "SimpleSkillDesc")]
    simple_skill_desc: String,
    #[serde(alias = "RatedSkillTreeID")]
    rated_skill_tree_id: Vec<u32>,
    #[serde(alias = "RatedRankID")]
    rated_rank_id: Vec<u32>,
    // WARN: unknown type, DM data is all empty
    #[serde(alias = "ExtraEffectIDList")]
    extra_effect_idlist: Vec<u32>,
    // WARN: unknown type, DM data is all empty
    #[serde(alias = "SimpleExtraEffectIDList")]
    simple_extra_effect_idlist: Vec<u32>,
    #[serde(alias = "ShowStanceList")]
    show_stance_list: Vec<Param>,
    // WARN: unknown type, DM data is all empty
    #[serde(alias = "ShowDamageList")]
    // WARN: unknown type, DM data is all empty
    show_damage_list: Vec<u32>,
    #[serde(alias = "ShowHealList")]
    show_heal_list: Vec<u32>,
    #[serde(alias = "InitCoolDown")]
    init_cool_down: i32,
    #[serde(alias = "CoolDown")]
    cool_down: i32,
    #[serde(alias = "SPBase")]
    spbase: Option<Param>,
    #[serde(alias = "SPNeed")]
    spneed: Option<Param>,
    #[serde(alias = "SPMultipleRatio")]
    spmultiple_ratio: Param,
    #[serde(alias = "BPNeed")]
    bpneed: Option<Param>,
    #[serde(alias = "BPAdd")]
    bpadd: Option<Param>,
    #[serde(alias = "SkillNeed")]
    skill_need: String,
    #[serde(alias = "DelayRatio")]
    delay_ratio: Param,
    #[serde(alias = "ParamList")]
    param_list: Vec<Vec<String>>,
    #[serde(alias = "SimpleParamList")]
    simple_param_list: Vec<Vec<Param>>,
    #[serde(alias = "StanceDamageType")]
    stance_damage_type: Option<Element>,
    #[serde(alias = "AttackType")]
    attack_type: Option<SkillType>,
    #[serde(alias = "SkillEffect")]
    skill_effect: SKillEffect,
    #[serde(alias = "SkillComboValueDelta")]
    skill_combo_value_delta: Option<Param>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpstreamAvatarSkillTreeConfig {
    #[serde(alias = "PointID")]
    point_id: u32,
    #[serde(alias = "Level")]
    level: u32,
    #[serde(alias = "AvatarID")]
    avatar_id: u32,
    #[serde(alias = "PointType")]
    point_type: u32,
    #[serde(alias = "Anchor")]
    anchor: Anchor,
    #[serde(alias = "MaxLevel")]
    max_level: u32,
    #[serde(alias = "DefaultUnlock")]
    default_unlock: Option<bool>,
    #[serde(alias = "PrePoint")]
    pre_point: Vec<u32>,
    #[serde(alias = "StatusAddList")]
    status_add_list: Vec<AbilityProperty>,
    #[serde(alias = "MaterialList")]
    material_list: Vec<MiniItem>,
    #[serde(alias = "AvatarPromotionLimit")]
    avatar_promotion_limit: Option<u32>,
    #[serde(alias = "LevelUpSkillID")]
    level_up_skill_id: Vec<u32>,
    #[serde(alias = "IconPath")]
    icon_path: AssetPath,
    #[serde(alias = "PointName")]
    point_name: HashedString,
    #[serde(alias = "PointDesc")]
    point_desc: HashedString,
    #[serde(alias = "AbilityName")]
    ability_name: String,
    #[serde(alias = "PointTriggerKey")]
    point_trigger_key: TextHash,
    #[serde(alias = "ParamList")]
    param_list: Vec<Param>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AvatarSkillTreeConfig {
    pub point_id: u32,
    pub level: Vec<u32>,
    pub avatar_id: u32,
    pub point_type: u32,
    pub anchor: Anchor,
    pub max_level: u32,
    pub default_unlock: bool,
    pub pre_point: Vec<u32>,
    pub status_add_list: Vec<AbilityProperty>,
    pub material_list: Vec<Vec<MiniItem>>,
    pub avatar_promotion_limit: Vec<u32>,
    pub level_up_skill_id: Vec<u32>,
    pub icon_path: AssetPath,
    pub point_name: String,
    pub point_desc: String,
    pub ability_name: String,
    pub point_trigger_key: String,
    pub param_list: Vec<Vec<f64>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
enum SKillEffect {
    SingleAttack,
    AoEAttack,
    MazeAttack,
    Blast,
    Impair,
    Bounce,
    Enhance,
    Support,
    Defence,
    Restore,
}

#[async_trait]
impl DbData for AvatarSkillConfig {
    type TUpstream = HashMap<u32, BTreeMap<u32, UpstreamAvatarSkillConfig>>;
    type TLocal = HashMap<u32, AvatarSkillConfig>;

    fn path_data() -> &'static str {
        "ExcelOutput/AvatarSkillConfig.json"
    }

    async fn upstream_convert(
        skill_db: HashMap<u32, BTreeMap<u32, UpstreamAvatarSkillConfig>>,
    ) -> Result<HashMap<u32, AvatarSkillConfig>, WorkerError> {
        let mut res: HashMap<u32, AvatarSkillConfig> = HashMap::new();
        let text_map: HashMap<String, String> = TextMap::read().await?;
        for (k, inner_map) in skill_db.into_iter() {
            let rest = inner_map.get(&1).unwrap().clone();
            let unsplitted_desc = rest.skill_desc.read_from_textmap(&text_map)?;

            // merge algorithms
            let (mut levels, mut param_lists, mut simple_param_lists) =
                (Vec::new(), Vec::new(), Vec::new());
            inner_map.iter().for_each(|(_, b)| {
                levels.push(b.level);
                let current_param: Vec<String> = get_sorted_params(
                    b.param_list.iter().map(|e| e.value).collect(),
                    &unsplitted_desc,
                )
                .iter()
                .map(|e| e.to_string())
                .collect();

                param_lists.push(current_param);
                simple_param_lists.push(b.simple_param_list.clone());
            });

            res.insert(
                k,
                AvatarSkillConfig {
                    skill_id: rest.skill_id,
                    skill_name: rest.skill_name.read_from_textmap(&text_map)?,
                    skill_tag: rest.skill_tag.read_from_textmap(&text_map)?,
                    skill_type_desc: rest.skill_type_desc.read_from_textmap(&text_map)?,
                    level: levels,
                    max_level: rest.max_level,
                    skill_trigger_key: rest.skill_trigger_key,
                    skill_icon: rest.skill_icon,
                    ultra_skill_icon: rest.ultra_skill_icon,
                    level_up_cost_list: rest.level_up_cost_list,
                    skill_desc: unsplitted_desc.into(),
                    simple_skill_desc: rest.simple_skill_desc.read_from_textmap(&text_map)?,
                    rated_skill_tree_id: rest.rated_skill_tree_id,
                    rated_rank_id: rest.rated_rank_id,
                    extra_effect_idlist: rest.extra_effect_idlist,
                    simple_extra_effect_idlist: rest.simple_extra_effect_idlist,
                    show_stance_list: rest.show_stance_list,
                    show_damage_list: rest.show_damage_list,
                    show_heal_list: rest.show_heal_list,
                    init_cool_down: rest.init_cool_down,
                    cool_down: rest.cool_down,
                    spbase: rest.spbase,
                    spneed: rest.spneed,
                    spmultiple_ratio: rest.spmultiple_ratio,
                    bpneed: rest.bpneed,
                    bpadd: rest.bpadd,
                    skill_need: rest.skill_need.read_from_textmap(&text_map)?,
                    delay_ratio: rest.delay_ratio,
                    param_list: param_lists,
                    simple_param_list: simple_param_lists,
                    stance_damage_type: rest.stance_damage_type,
                    attack_type: rest.attack_type,
                    skill_effect: rest.skill_effect,
                    skill_combo_value_delta: rest.skill_combo_value_delta,
                },
            );
        }
        Ok(res)
    }
}

impl AvatarSkillConfig {
    /// write an `AvatarSkillConfig` to smaller chunks in `tmp`
    pub async fn write_splitted() -> Result<(), WorkerError> {
        let skill_db: HashMap<u32, AvatarSkillConfig> = AvatarSkillConfig::read().await?;
        std::fs::create_dir_all("/tmp/AvatarSkillConfigs")?;
        for (key, value) in skill_db.into_iter() {
            let filepath = format!("/tmp/AvatarSkillConfigs/{}.json", key);
            let json_blob = serde_json::to_string(&value)?;
            // save to a new file
            std::fs::write(filepath, json_blob)?;
        }
        Ok(())
    }

    pub fn read_splitted_by_skillid(skill_id: u32) -> Result<Self, WorkerError> {
        let filepath = format!("/tmp/AvatarSkillConfigs/{}.json", skill_id);
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);
        let data: Self = serde_json::from_reader(reader)?;
        Ok(data)
    }
}

#[async_trait]
impl DbData for AvatarSkillTreeConfig {
    type TUpstream = BTreeMap<u32, BTreeMap<u32, UpstreamAvatarSkillTreeConfig>>;
    type TLocal = BTreeMap<u32, AvatarSkillTreeConfig>;

    fn path_data() -> &'static str {
        "ExcelOutput/AvatarSkillTreeConfig.json"
    }
    async fn upstream_convert(
        tracetree_db: BTreeMap<u32, BTreeMap<u32, UpstreamAvatarSkillTreeConfig>>,
    ) -> Result<BTreeMap<u32, AvatarSkillTreeConfig>, WorkerError> {
        let text_map = TextMap::read().await?;

        let transformed = tracetree_db
            .into_iter()
            .map(|(key, value)| {
                let converted_value = raw_convert(value, &text_map);
                (key, converted_value)
            })
            .collect();

        Ok(transformed)
    }
}

fn raw_convert(
    value: BTreeMap<u32, UpstreamAvatarSkillTreeConfig>,
    text_map: &HashMap<String, String>,
) -> AvatarSkillTreeConfig {
    let default_first = value.get(&1).unwrap().clone();

    let levels: Vec<u32> = value.values().map(|big| big.level).collect();
    let mats: Vec<Vec<MiniItem>> = value
        .values()
        .map(|big| big.material_list.clone())
        .collect();
    let promotion_limits: Vec<u32> = value
        .values()
        .map(|big| big.avatar_promotion_limit.unwrap_or_default())
        .collect();
    let params: Vec<Vec<f64>> = value
        .values()
        .map(|big| big.param_list.iter().map(|e| e.value).collect())
        .collect();

    AvatarSkillTreeConfig {
        point_id: default_first.point_id,
        level: levels,
        avatar_id: default_first.avatar_id,
        point_type: default_first.point_type,
        anchor: default_first.anchor,
        max_level: default_first.max_level,
        default_unlock: default_first.default_unlock.unwrap_or(false),
        pre_point: default_first.pre_point,
        status_add_list: default_first.status_add_list,
        material_list: mats,
        avatar_promotion_limit: promotion_limits,
        level_up_skill_id: default_first.level_up_skill_id,
        icon_path: default_first.icon_path,
        point_name: default_first
            .point_name
            .dehash(text_map)
            .unwrap_or_default(),
        point_desc: default_first
            .point_desc
            .dehash(text_map)
            .unwrap_or_default(),
        ability_name: default_first.ability_name,
        point_trigger_key: default_first
            .point_trigger_key
            .read_from_textmap(text_map)
            .unwrap_or_default(),
        param_list: params,
    }
}

// BUG: not actually working atm
impl AvatarSkillTreeConfig {
    pub async fn write_splitted() -> Result<(), WorkerError> {
        let tracetree_db: BTreeMap<u32, Self> = Self::read().await?;
        std::fs::create_dir_all("/tmp/AvatarSkillTreeConfigs")?;
        for (key, value) in tracetree_db.into_iter() {
            let filepath = format!("/tmp/AvatarSkillTreeConfigs/{}.json", key);
            let json_blob = serde_json::to_string(&value)?;
            // save to a new file
            std::fs::write(filepath, json_blob)?;
        }
        Ok(())
    }

    pub fn read_splitted_by_skillid(skill_id: u32) -> Result<Self, WorkerError> {
        let filepath = format!("/tmp/AvatarSkillTreeConfigs/{}.json", skill_id);
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);
        let data: Self = serde_json::from_reader(reader)?;
        Ok(data)
    }
}
