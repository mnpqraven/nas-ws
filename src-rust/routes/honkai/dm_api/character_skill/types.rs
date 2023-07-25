use crate::{
    handler::error::WorkerError,
    routes::honkai::{
        dm_api::{
            desc_param::{get_sorted_params, ParameterizedDescription},
            hash::TextHash,
            types::{Param, TextMap},
        },
        mhy_api::{
            internal::categorizing::SkillType,
            types_parsed::shared::{AssetPath, Element},
        },
        traits::DbData,
    },
};
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

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
