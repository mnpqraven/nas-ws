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
    pub name: String,
    pub version: String,
    pub date_start: DateTime<Utc>,
    pub date_end: DateTime<Utc>,
}
#[derive(Serialize, JsonResponse, Clone, Debug)]
pub struct PatchList {
    patches: Vec<Patch>,
}

impl Patch {
    const BASE_1_1: (i32, u32, u32, u32, u32, u32) = (2023, 6, 7, 2, 0, 0);
    pub fn base() -> Self {
        let (year, month, day, hour, min, sec) = Self::BASE_1_1;
        let start_date = Utc
            .with_ymd_and_hms(year, month, day, hour, min, sec)
            .unwrap();
        Self::new("Galatic Roaming", "1.1", start_date)
    }

    /// Get the start, middle, end date of a patch
    pub fn get_patch_boundaries(
        current_date: DateTime<Utc>,
    ) -> (DateTime<Utc>, DateTime<Utc>, DateTime<Utc>) {
        let base_1_1 = Utc.with_ymd_and_hms(2023, 6, 7, 2, 0, 0).unwrap();
        let (mut l_bound, mut m_bound, mut r_bound) = (
            base_1_1,
            base_1_1 + Duration::weeks(3),
            base_1_1 + Duration::weeks(6),
        );
        while r_bound < current_date {
            l_bound = r_bound;
            m_bound += Duration::weeks(3);
            r_bound += Duration::weeks(6);
        }
        (l_bound, m_bound, r_bound)
    }

    pub fn contains(&self, date: DateTime<Utc>) -> bool {
        self.date_start <= date && self.date_end >= date
    }

    /// get the next timeslot of a future patch
    /// WARN: the name and version is not (yet) edited
    pub fn next(&mut self) {
        self.date_start += Duration::weeks(6);
        self.date_end += Duration::weeks(6);
    }

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

        let mut amount: u32 = 0;
        while next_bp_start < to_date {
            amount += 1;
            next_bp_start += Duration::weeks(6);
        }
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
