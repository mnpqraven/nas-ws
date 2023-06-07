use crate::handler::{error::WorkerError, FromAxumResponse};
use axum::Json;
use chrono::{DateTime, Duration, TimeZone, Utc};
use response_derive::JsonResponse;
use serde::Serialize;
use tracing::debug;
use vercel_runtime::{Body, Response, StatusCode};

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
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

    pub fn patches_passed_number(to_date: DateTime<Utc>) -> u32 {
        let mut next_bp_start = Utc.with_ymd_and_hms(2023, 6, 7, 2, 0, 0).unwrap();
        // get next bp start date (next patch)
        while Utc::now() > next_bp_start {
            next_bp_start += Duration::weeks(6);
        }
        tracing::info!("{:?}", next_bp_start);

        let mut amount: u32 = 0;
        while next_bp_start < to_date {
            amount += 1;
            next_bp_start += Duration::weeks(6);
        }
        tracing::info!(amount);
        amount
    }
}

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
