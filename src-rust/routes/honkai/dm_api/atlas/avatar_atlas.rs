use super::serialize_date_string;
use crate::routes::honkai::{
    dm_api::types::Hash,
    mhy_api::internal::impls::{DbData, DbDataLike},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
    pub cv_cn: Hash,
    #[serde(alias = "CV_JP")]
    pub cv_jp: Hash,
    #[serde(alias = "CV_KR")]
    pub cv_kr: Hash,
    #[serde(alias = "CV_EN")]
    pub cv_en: Hash,
    #[serde(alias = "CampID")]
    pub camp_id: u32,
}

impl<T: DbDataLike> DbData<T> for UpstreamAvatarAtlas {
    fn path_data() -> (&'static str, &'static str) {
        (AVATAR_ATLAS_LOCAL, AVATAR_ATLAS_REMOTE)
    }
}
