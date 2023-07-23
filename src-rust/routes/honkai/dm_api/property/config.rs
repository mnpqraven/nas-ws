use crate::{
    handler::error::WorkerError,
    routes::honkai::{
        dm_api::{hash::TextHash, types::TextMap},
        traits::DbData,
    },
};
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpstreamAvatarPropertyConfig {
    #[serde(alias = "PropertyType")]
    property_type: String,
    #[serde(alias = "PropertyName")]
    property_name: TextHash,
    #[serde(alias = "PropertyNameSkillTree")]
    property_name_skill_tree: TextHash,
    #[serde(alias = "PropertyNameRelic")]
    property_name_relic: TextHash,
    #[serde(alias = "PropertyNameFilter")]
    property_name_filter: TextHash,

    #[serde(alias = "MainRelicFilter")]
    main_relic_filter: Option<u32>,
    #[serde(alias = "SubRelicFilter")]
    sub_relic_filter: Option<u32>,
    #[serde(alias = "PropertyClassify")]
    property_classify: Option<u32>,
    #[serde(alias = "IsDisplay")]
    is_display: Option<bool>,
    #[serde(alias = "isBattleDisplay")]
    is_battle_display: Option<bool>,
    #[serde(alias = "Order")]
    order: i32,
    #[serde(alias = "IconPath")]
    icon_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AvatarPropertyConfig {
    property_type: String,
    property_name: String,
    property_name_skill_tree: String,
    property_name_relic: String,
    property_name_filter: String,
    main_relic_filter: Option<u32>,
    sub_relic_filter: Option<u32>,
    property_classify: Option<u32>,
    is_display: Option<bool>,
    is_battle_display: Option<bool>,
    order: i32,
    icon_path: String,
}

#[async_trait]
impl DbData for AvatarPropertyConfig {
    type TUpstream = HashMap<String, UpstreamAvatarPropertyConfig>;
    type TLocal = HashMap<String, AvatarPropertyConfig>;

    fn path_data() -> &'static str {
        "ExcelOutput/AvatarPropertyConfig.json"
    }

    async fn upstream_convert(
        from: HashMap<String, UpstreamAvatarPropertyConfig>,
    ) -> Result<HashMap<String, AvatarPropertyConfig>, WorkerError> {
        let text_map = TextMap::read().await?;

        let transformed = from
            .into_iter()
            .map(|(k, v)| {
                let data = AvatarPropertyConfig {
                    property_type: v.property_type,
                    property_name: v
                        .property_name
                        .read_from_textmap(&text_map)
                        .unwrap_or_default(),
                    property_name_skill_tree: v
                        .property_name_skill_tree
                        .read_from_textmap(&text_map)
                        .unwrap_or_default(),
                    property_name_relic: v
                        .property_name_relic
                        .read_from_textmap(&text_map)
                        .unwrap_or_default(),
                    property_name_filter: v
                        .property_name_filter
                        .read_from_textmap(&text_map)
                        .unwrap_or_default(),
                    is_display: v.is_display,
                    is_battle_display: v.is_battle_display,
                    order: v.order,
                    icon_path: v.icon_path,
                    main_relic_filter: v.main_relic_filter,
                    sub_relic_filter: v.sub_relic_filter,
                    property_classify: v.property_classify,
                };
                (k, data)
            })
            .collect();
        Ok(transformed)
    }
}
