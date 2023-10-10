use crate::{
    handler::error::WorkerError,
    routes::honkai::{
        dm_api::{
            character::types::MiniItem,
            desc_param::{get_sorted_params, ParameterizedDescription},
            hash::{HashedString, TextHash},
            types::{AbilityProperty, Anchor, AssetPath, Param, TextMap},
        },
        traits::DbData,
    },
};
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpstreamSkillTreeConfig {
    #[serde(alias = "PointID")]
    point_id: u32,
    #[serde(alias = "Level")]
    level: u32,
    #[serde(alias = "AvatarID")]
    avatar_id: u32,
    #[serde(alias = "PointType")]
    point_type: u32,
    #[serde(alias = "PrePoint")]
    pre_point: Vec<u32>,
    #[serde(alias = "Anchor")]
    anchor: Anchor,
    #[serde(alias = "MaxLevel")]
    max_level: u32,
    #[serde(alias = "DefaultUnlock")]
    default_unlock: Option<bool>,
    #[serde(alias = "StatusAddList")]
    status_add_list: Vec<AbilityProperty>,
    #[serde(alias = "MaterialList")]
    material_list: Vec<MiniItem>,
    #[serde(alias = "AvatarPromotionLimit")]
    pub avatar_promotion_limit: Option<u32>,
    #[serde(alias = "LevelUpSkillID")]
    level_up_skill_id: Vec<u32>,
    #[serde(alias = "IconPath")]
    pub icon_path: AssetPath,
    #[serde(alias = "PointName")]
    point_name: HashedString,
    #[serde(alias = "PointDesc")]
    point_desc: HashedString,
    #[serde(alias = "AbilityName")]
    ability_name: HashedString,
    #[serde(alias = "PointTriggerKey")]
    point_trigger_key: TextHash,
    #[serde(alias = "ParamList")]
    pub param_list: Vec<Param>,
}

// TODO: depecrate
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct SkillTreeConfig {
    point_id: u32,
    level: Vec<u32>,
    pub avatar_id: u32,
    point_type: u32,
    pre_point: Vec<u32>,
    anchor: Anchor,
    max_level: u32,
    default_unlock: Vec<bool>,
    status_add_list: Vec<AbilityProperty>,
    material_list: Vec<Vec<MiniItem>>,
    pub avatar_promotion_limit: Vec<Option<u32>>,
    level_up_skill_id: Vec<u32>,
    pub icon_path: AssetPath,
    point_name: String,
    point_desc: ParameterizedDescription,
    ability_name: String,
    point_trigger_key: String,
    pub param_list: Vec<String>, // TODO: own type
}

#[async_trait]
impl DbData for SkillTreeConfig {
    type TUpstream = HashMap<u32, BTreeMap<u32, UpstreamSkillTreeConfig>>;
    type TLocal = HashMap<u32, SkillTreeConfig>;

    fn path_data() -> &'static str {
        "ExcelOutput/AvatarSkillTreeConfig.json"
    }

    async fn upstream_convert(
        from: HashMap<u32, BTreeMap<u32, UpstreamSkillTreeConfig>>,
    ) -> Result<HashMap<u32, SkillTreeConfig>, WorkerError> {
        let text_map: HashMap<String, String> = TextMap::read().await?;

        let transformed = from
            .into_iter()
            .map(|(k, inner_map)| {
                let rest = inner_map.get(&1).unwrap();
                let unsplitted_desc = TextHash::from(rest.point_desc.clone())
                    .read_from_textmap(&text_map)
                    .unwrap_or_default();

                let sorted_params: Vec<String> = get_sorted_params(
                    rest.param_list.iter().map(|e| e.value).collect(),
                    &unsplitted_desc,
                )
                .iter()
                .map(|e| e.to_string())
                .collect();

                // merge algorithms
                let (mut levels, mut default_unlocks, mut material_lists, mut promotion_limits) =
                    (Vec::new(), Vec::new(), Vec::new(), Vec::new());
                inner_map.iter().for_each(|(_, b)| {
                    levels.push(b.level);
                    default_unlocks.push(b.default_unlock.unwrap_or(false));
                    material_lists.push(b.material_list.clone());
                    promotion_limits.push(b.avatar_promotion_limit);
                });

                let transformed = SkillTreeConfig {
                    point_id: rest.point_id,
                    level: levels,
                    avatar_id: rest.avatar_id,
                    point_type: rest.point_type,
                    pre_point: rest.pre_point.clone(),
                    anchor: rest.anchor.clone(),
                    max_level: rest.max_level,
                    default_unlock: default_unlocks,
                    status_add_list: rest.status_add_list.clone(),
                    material_list: material_lists,
                    avatar_promotion_limit: promotion_limits,
                    level_up_skill_id: rest.level_up_skill_id.clone(),
                    icon_path: rest.icon_path.clone(),
                    point_name: rest.point_name.dehash(&text_map).unwrap_or_default(),
                    point_desc: unsplitted_desc.into(),
                    ability_name: rest.ability_name.dehash(&text_map).unwrap_or_default(),
                    point_trigger_key: rest
                        .point_trigger_key
                        .read_from_textmap(&text_map)
                        .unwrap_or_default(),
                    param_list: sorted_params,
                };
                (k, transformed)
            })
            .collect();
        Ok(transformed)
    }
}

#[cfg(test)]
mod tests {
    use crate::routes::honkai::{
        dm_api::equipment_skill::skill_tree_config::SkillTreeConfig, traits::DbData,
    };

    #[tokio::test]
    async fn read() {
        let trace_db = SkillTreeConfig::read().await;
        assert!(trace_db.is_ok());
    }
}
