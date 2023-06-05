use crate::handler::error::WorkerError;
use axum::{extract::rejection::JsonRejection, Json};
use chrono::{DateTime, Duration, TimeZone, Utc};
use tracing::{error, info};

use super::types::{EstimateCfg, Rewards};

pub async fn jade_estimate(
    rpayload: Result<Json<EstimateCfg>, JsonRejection>,
) -> Result<Json<Rewards>, WorkerError> {
    // std::thread::sleep(std::time::Duration::from_secs(5));
    if let Ok(Json(payload)) = rpayload {
        Ok(Json(Rewards::from_cfg(payload)))
    } else {
        let err = rpayload.unwrap_err();
        error!("{}", err.body_text());
        Err(WorkerError::ParseData(err.body_text()))
    }
}

/// [TODO:description]
///
/// * `from_date`: [TODO:parameter]
/// * `to_date`: [TODO:parameter]
/// returns a tuple of differences in days and weeks, week diff is always rounded up (e.g a difference of 17-18 days would equate to 3 weeks)
pub fn get_date_differences(
    from_date: Option<DateTime<Utc>>,
    to_date: DateTime<Utc>,
) -> (u32, i64) {
    let dt_now = from_date.unwrap_or(Utc::now());
    let diff = to_date - dt_now;

    let diff_days = diff.num_days() as u32;
    let diff_weeks = match i64::from(diff_days / 7) > diff.num_weeks() {
        true => diff.num_weeks() + 1,
        false => diff.num_weeks(),
    };
    (diff_days, diff_weeks)
}

pub(super) fn get_current_patch_boundaries(
    current_date: DateTime<Utc>,
) -> (DateTime<Utc>, DateTime<Utc>) {
    let base_1_1 = Utc.with_ymd_and_hms(2023, 6, 7, 2, 0, 0).unwrap();
    let (mut l_bound, mut r_bound) = (base_1_1, base_1_1 + Duration::weeks(6));
    while r_bound < current_date {
        l_bound = r_bound;
        r_bound += Duration::weeks(6);
    }
    (l_bound, r_bound)
}
