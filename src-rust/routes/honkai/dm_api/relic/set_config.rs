use crate::{
    handler::error::WorkerError,
    routes::honkai::{
        dm_api::{
            hash::TextHash,
            types::{AssetPath, TextMap},
        },
        traits::DbData,
    },
};
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpstreamRelicSetConfig {
    #[serde(alias = "SetID")]
    set_id: u32,
    #[serde(alias = "SetSkillList")]
    set_skill_list: Vec<u32>,
    #[serde(alias = "SetIconPath")]
    set_icon_path: AssetPath,
    #[serde(alias = "SetIconFigurePath")]
    set_icon_figure_path: AssetPath,
    #[serde(alias = "SetName")]
    set_name: TextHash,
    #[serde(alias = "Release")]
    release: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct RelicSetConfig {
    pub set_id: u32,
    pub set_skill_list: Vec<u32>,
    pub set_icon_path: AssetPath,
    pub set_icon_figure_path: AssetPath,
    pub set_name: String,
    pub release: Option<bool>,
}

#[async_trait]
impl DbData for RelicSetConfig {
    type TUpstream = HashMap<u32, UpstreamRelicSetConfig>;
    type TLocal = HashMap<u32, RelicSetConfig>;

    fn path_data() -> &'static str {
        "ExcelOutput/RelicSetConfig.json"
    }

    async fn upstream_convert(
        from: HashMap<u32, UpstreamRelicSetConfig>,
    ) -> Result<HashMap<u32, RelicSetConfig>, WorkerError> {
        let text_map = TextMap::read().await?;
        let res = from
            .into_iter()
            .map(|(k, v)| {
                let value = RelicSetConfig {
                    set_id: v.set_id,
                    set_skill_list: v.set_skill_list,
                    set_icon_path: v.set_icon_path,
                    set_icon_figure_path: v.set_icon_figure_path,
                    set_name: v.set_name.read_from_textmap(&text_map).unwrap_or_default(),
                    release: v.release,
                };
                (k, value)
            })
            .collect();

        Ok(res)
    }
}
