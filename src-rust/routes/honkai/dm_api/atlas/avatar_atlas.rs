use std::collections::HashMap;

use crate::{routes::honkai::{dm_api::hash::TextHash, traits::DbData}, handler::error::WorkerError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpstreamAvatarAtlas {
    #[serde(alias = "AvatarID")]
    pub avatar_id: u32,
    // #[serde(alias = "GachaSchedule")]
    // #[serde(deserialize_with = "serialize_date_string")]
    // pub gacha_schedule: Option<DateTime<Utc>>,
    // #[serde(alias = "IsLocalTime")]
    // pub is_local_time: Option<bool>,
    #[serde(alias = "CV_CN")]
    pub cv_cn: TextHash,
    #[serde(alias = "CV_JP")]
    pub cv_jp: TextHash,
    #[serde(alias = "CV_KR")]
    pub cv_kr: TextHash,
    #[serde(alias = "CV_EN")]
    pub cv_en: TextHash,
    #[serde(alias = "CampID")]
    pub camp_id: u32,
}

#[async_trait]
impl DbData for UpstreamAvatarAtlas {
    type TUpstream = HashMap<u32, UpstreamAvatarAtlas>;
    type TLocal = HashMap<u32, UpstreamAvatarAtlas>;

    fn path_data() -> &'static str {
        "ExcelOutput/AvatarAtlas.json"
    }

    async fn upstream_convert(from: Self::TUpstream) -> Result<Self::TLocal, WorkerError> {
        Ok(from)
    }
}
