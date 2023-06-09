use super::serialize_date_string;
use crate::routes::honkai::dm_api::hash::TextHash;
use crate::routes::honkai::mhy_api::internal::impls::{DbData, DbDataLike};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(target_os = "windows")]
const AVATAR_ATLAS_LOCAL: &str = "c:\\tmp\\avatar_atlas.json";
#[cfg(target_os = "linux")]
const AVATAR_ATLAS_LOCAL: &str = "/tmp/avatar_atlas.json";

const AVATAR_ATLAS_REMOTE: &str =
    "https://raw.githubusercontent.com/Dimbreath/StarRailData/master/ExcelOutput/AvatarAtlas.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(super) struct UpstreamAvatarAtlas {
    #[serde(alias = "AvatarID")]
    pub avatar_id: u32,
    #[serde(alias = "GachaSchedule")]
    #[serde(deserialize_with = "serialize_date_string")]
    pub gacha_schedule: Option<DateTime<Utc>>,
    #[serde(alias = "IsLocalTime")]
    pub is_local_time: Option<bool>,
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

impl<T: DbDataLike> DbData<T> for UpstreamAvatarAtlas {
    fn path_data() -> (&'static str, &'static str) {
        (AVATAR_ATLAS_LOCAL, AVATAR_ATLAS_REMOTE)
    }
}
