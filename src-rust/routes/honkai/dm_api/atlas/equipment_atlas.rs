use super::serialize_date_string;
use crate::routes::honkai::traits::{DbData, DbDataLike};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(target_os = "windows")]
const EQUIPMENT_ATLAS_LOCAL: &str = "c:\\tmp\\equipment_atlas.json";
#[cfg(target_os = "linux")]
const EQUIPMENT_ATLAS_LOCAL: &str = "/tmp/equipment_atlas.json";

const EQUIPMENT_ATLAS_REMOTE: &str =
    "https://raw.githubusercontent.com/Dimbreath/StarRailData/master/ExcelOutput/EquipmentAtlas.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(super) struct UpstreamEquipmentAtlas {
    #[serde(alias = "EquipmentID")]
    pub equipment_id: u32,
    #[serde(alias = "GachaSchedule")]
    #[serde(deserialize_with = "serialize_date_string")]
    pub gacha_schedule: Option<DateTime<Utc>>,
    #[serde(alias = "IsLocalTime")]
    pub is_local_time: Option<bool>,
}

impl<T: DbDataLike> DbData<T> for UpstreamEquipmentAtlas {
    fn path_data() -> (&'static str, &'static str) {
        (EQUIPMENT_ATLAS_LOCAL, EQUIPMENT_ATLAS_REMOTE)
    }
}
