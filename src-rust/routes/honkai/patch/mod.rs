use self::types::{Patch, PatchList};
use crate::{handler::error::WorkerError, routes::honkai::patch::types::PatchInfo};
use axum::Json;
use chrono::{TimeZone, Utc};
use tracing::debug;

pub mod types;

pub async fn list_future_patch_date() -> Result<Json<PatchList>, WorkerError> {
    let dt_1_1 = Utc.with_ymd_and_hms(2023, 6, 7, 2, 0, 0).unwrap();
    let patch_1_1 = Patch::new("Galatic Roaming", "1.1", dt_1_1);

    let patches: Vec<PatchInfo> = vec![
        PatchInfo("Patch 1.2".into(), "1.2".into()),
        PatchInfo("Patch 1.3".into(), "1.3".into()),
        PatchInfo("Patch 1.4".into(), "1.4".into()),
    ];

    let future_patches = PatchList::calculate_from_base(patch_1_1, patches);
    debug!("{:?}", future_patches);
    Ok(Json(future_patches))
}
