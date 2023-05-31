use crate::handler::{error::WorkerError, FromAxumResponse};
use axum::Json;
use chrono::{DateTime, Duration, TimeZone, Utc};
use response_derive::JsonResponse;
use serde::Serialize;
use tracing::debug;
use vercel_runtime::{Body, Response, StatusCode};

#[derive(Serialize, Clone, Debug)]
pub struct Patch {
    name: String,
    version: String,
    date_start: DateTime<Utc>,
    date_end: DateTime<Utc>,
}
#[derive(Serialize, JsonResponse, Clone, Debug)]
pub struct PatchList {
    patches: Vec<Patch>,
}

impl Patch {
    fn new(name: impl Into<String>, version: impl Into<String>, start_date: DateTime<Utc>) -> Self {
        let end_date = start_date + Duration::weeks(6);
        Self {
            name: name.into(),
            version: version.into(),
            date_start: start_date,
            date_end: end_date,
        }
    }
}

// name and version in Patch
struct PatchInfo(String, String);
impl PatchList {
    fn calculate_from_base(base_version: Patch, future_patches: Vec<PatchInfo>) -> Self {
        let mut res: Vec<Patch> = vec![base_version.clone()];
        let mut next_start_date = base_version.date_end;
        for PatchInfo(name, version) in future_patches.iter() {
            res.push(Patch::new(name, version, next_start_date));
            next_start_date += Duration::weeks(6);
        }

        match Utc::now() > base_version.date_start {
            true => {
                res.remove(0);
                Self { patches: res }
            }
            false => Self { patches: res },
        }
    }
}

pub async fn list_future_patch_date() -> Result<Json<PatchList>, WorkerError> {
    let dt_1_1 = Utc.with_ymd_and_hms(2023, 6, 7, 2, 0, 0).unwrap();
    let patch_1_1 = Patch::new("Galatic Roaming", "1.1", dt_1_1);
    let dt_now = Utc::now();
    let diff = dt_1_1 - dt_now;

    let patches: Vec<PatchInfo> = vec![
        PatchInfo("1.2".into(), "Patch 1.2".into()),
        PatchInfo("1.3".into(), "Patch 1.3".into()),
    ];

    let future_patches = PatchList::calculate_from_base(patch_1_1, patches);
    debug!("{:?}", future_patches);
    Ok(Json(future_patches))
}
