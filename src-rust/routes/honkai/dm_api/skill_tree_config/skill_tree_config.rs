use crate::{
    handler::error::WorkerError,
    routes::honkai::{
        dm_api::{
            avatar_config::upstream_avatar_config::Item,
            desc_param::{get_sorted_params, ParameterizedDescription},
            hash::{HashedString, TextHash},
            types::{AbilityProperty, Param, TextMap},
        },
        mhy_api::{internal::categorizing::Anchor, types_parsed::shared::AssetPath},
        traits::{DbData, DbDataLike},
    },
};
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[cfg(target_os = "windows")]
const SKILL_TREE_CONFIG_LOCAL: &str = "c:\\tmp\\avatar_skill_tree_config.json";
#[cfg(target_os = "linux")]
const SKILL_TREE_CONFIG_LOCAL: &str = "/tmp/avatar_skill_tree_config.json";

const SKILL_TREE_CONFIG_REMOTE: &str =
    "https://raw.githubusercontent.com/Dimbreath/StarRailData/master/ExcelOutput/AvatarSkillTreeConfig.json";

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
    material_list: Vec<Item>,
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

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename(serialize = "camelCase"))]
pub struct SkillTreeConfig {
    #[serde(alias = "PointID")]
    point_id: u32,
    #[serde(alias = "Level")]
    level: Vec<u32>,
    #[serde(alias = "AvatarID")]
    pub avatar_id: u32,
    #[serde(alias = "PointType")]
    point_type: u32,
    #[serde(alias = "PrePoint")]
    pre_point: Vec<u32>,
    #[serde(alias = "Anchor")]
    anchor: Anchor,
    #[serde(alias = "MaxLevel")]
    max_level: u32,
    #[serde(alias = "DefaultUnlock")]
    default_unlock: Vec<bool>,
    #[serde(alias = "StatusAddList")]
    status_add_list: Vec<AbilityProperty>,
    #[serde(alias = "MaterialList")]
    material_list: Vec<Vec<Item>>,
    #[serde(alias = "AvatarPromotionLimit")]
    pub avatar_promotion_limit: Vec<Option<u32>>,
    #[serde(alias = "LevelUpSkillID")]
    level_up_skill_id: Vec<u32>,
    #[serde(alias = "IconPath")]
    pub icon_path: AssetPath,
    #[serde(alias = "PointName")]
    point_name: String,
    #[serde(alias = "PointDesc")]
    point_desc: ParameterizedDescription,
    #[serde(alias = "AbilityName")]
    ability_name: String,
    #[serde(alias = "PointTriggerKey")]
    point_trigger_key: String,
    #[serde(alias = "ParamList")]
    pub param_list: Vec<String>, // TODO: own type
}

#[async_trait]
impl<T: DbDataLike> DbData<T> for SkillTreeConfig {
    fn path_data() -> (&'static str, &'static str) {
        (SKILL_TREE_CONFIG_LOCAL, SKILL_TREE_CONFIG_REMOTE)
    }

    async fn try_write_disk() -> Result<String, WorkerError> {
        let mut res: HashMap<String, SkillTreeConfig> = HashMap::new();
        let text_map: HashMap<String, String> = TextMap::read().await?;

        let trace_db = reqwest::get(SKILL_TREE_CONFIG_REMOTE).await?.text().await?;
        // WARN: BTreeMap authenticity check
        let trace_db: HashMap<String, BTreeMap<u32, UpstreamSkillTreeConfig>> =
            serde_json::from_str(&trace_db)?;

        for (k, inner_map) in trace_db.into_iter() {
            let rest = inner_map.get(&1).unwrap();
            let unsplitted_desc =
                TextHash::from(rest.point_desc.clone()).read_from_textmap(&text_map)?;
            if inner_map.len() > 1 {
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

                res.insert(
                    k,
                    SkillTreeConfig {
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
                        point_name: rest.point_name.dehash(&text_map)?,
                        point_desc: unsplitted_desc.into(),
                        ability_name: rest.ability_name.dehash(&text_map)?,
                        point_trigger_key: rest.point_trigger_key.read_from_textmap(&text_map)?,
                        param_list: sorted_params,
                    },
                );
            } else if let Some(value) = inner_map.get(&1) {
                let sorted_params: Vec<String> = get_sorted_params(
                    rest.param_list.iter().map(|e| e.value).collect(),
                    &unsplitted_desc,
                )
                .iter()
                .map(|e| e.to_string())
                .collect();

                let value_into = SkillTreeConfig {
                    point_id: value.point_id,
                    level: vec![value.level],
                    avatar_id: value.avatar_id,
                    point_type: value.point_type,
                    pre_point: value.pre_point.clone(),
                    anchor: value.anchor.clone(),
                    max_level: value.max_level,
                    default_unlock: vec![value.default_unlock.unwrap_or(false)],
                    status_add_list: value.status_add_list.clone(),
                    material_list: vec![value.material_list.clone()],
                    avatar_promotion_limit: vec![value.avatar_promotion_limit],
                    level_up_skill_id: value.level_up_skill_id.clone(),
                    icon_path: value.icon_path.clone(),
                    point_name: rest.point_name.dehash(&text_map)?,
                    point_desc: unsplitted_desc.into(),
                    ability_name: rest.ability_name.dehash(&text_map)?,
                    point_trigger_key: value.point_trigger_key.read_from_textmap(&text_map)?,
                    param_list: sorted_params,
                };
                res.insert(k, value_into);
            }
        }
        let data = serde_json::to_string_pretty(&res)?;
        std::fs::write(SKILL_TREE_CONFIG_LOCAL, &data)?;
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use crate::routes::honkai::{
        dm_api::skill_tree_config::skill_tree_config::SkillTreeConfig, traits::DbData,
    };

    #[tokio::test]
    async fn read() {
        let trace_db = <SkillTreeConfig as DbData<SkillTreeConfig>>::read().await;
        assert!(trace_db.is_ok());
    }
}
