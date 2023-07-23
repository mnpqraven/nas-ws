use crate::{handler::error::WorkerError, routes::honkai::traits::DbData};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpstreamEquipmentAtlas {
    #[serde(alias = "EquipmentID")]
    pub equipment_id: u32,
}

#[async_trait]
impl DbData for UpstreamEquipmentAtlas {
    type TUpstream = HashMap<u32, UpstreamEquipmentAtlas>;
    type TLocal = HashMap<u32, UpstreamEquipmentAtlas>;

    fn path_data() -> &'static str {
        "ExcelOutput/EquipmentAtlas.json"
    }

    async fn upstream_convert(from: Self::TUpstream) -> Result<Self::TLocal, WorkerError> {
        Ok(from)
    }
}
