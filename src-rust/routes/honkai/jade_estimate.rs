use crate::{handler::error::WorkerError, routes::honkai::types::Server};
use axum::{extract::rejection::JsonRejection, Json};
use chrono::{DateTime, Datelike, Duration, TimeZone, Timelike, Utc, Weekday};
use tracing::{error, info};

use super::types::{DateRange, EstimateCfg, Rewards};

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

/// Get a difference in days and weeks between 2 dates
///
/// WARN: TESTING IS NEEDED FOR WEEK DIFF
///
/// returns a tuple of differences in days and weeks, week diff is always rounded up (e.g a difference of 17-18 days would equate to 3 weeks)
pub fn get_date_differences(server: &Server, to_date: DateTime<Utc>) -> (u32, i64) {
    // BUG: querying (when late at night/less than 24 hours until the next
    // reset) next day will give a diff day of 0 but we should still receive
    // 1 daily reward
    // either change parent logic or increment this day diff by 1. e.g:
    // query @03:00 UTC 1/1, server reset @9:00 UTC, goal 2/1
    // -> diff_days = 2
    // query @14:00 UTC 1/1, server reset @9:00 UTC, goal 2/1
    // -> diff_days = 1
    // TODO: optional arg in fn call
    let dt_now = Utc::now();

    let today_right_after_reset = |a: &DateTime<Utc>, server: &Server| {
        let mut res = Utc
            .with_ymd_and_hms(
                a.year(),
                a.month(),
                a.day(),
                server.get_utc_reset_hour(),
                1,
                0,
            )
            .unwrap();
        // can't forward, has to rewind by 1 day
        if res > *a {
            res -= Duration::days(1);
        }
        res
    };

    // let diff = to_date - dt_now;
    let (mut diff_days, mut diff_weeks) = (0, 0);

    for current_date in DateRange(today_right_after_reset(&dt_now, server), to_date) {
        diff_days += 1;

        // TODO: TESTING
        if current_date.weekday() == Weekday::Mon {
            diff_weeks += 1;
        }
    }

    (diff_days as u32, diff_weeks)
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

#[cfg(test)]
mod test {
    use super::get_date_differences;
    use crate::routes::honkai::types::{
        EqTier::Zero, EstimateCfg, RailPassCfg, Rewards, Server::America, SimpleDate,
    };

    #[test]
    fn wtf() {
        let cfg = EstimateCfg {
            server: America,
            until_date: SimpleDate {
                day: 5,
                month: 6,
                year: 2023,
            },
            rail_pass: RailPassCfg {
                use_rail_pass: true,
                days_left: Some(30),
            },
            battle_pass: false,
            eq: Zero,
            current_rolls: Some(0),
            current_jades: None,
        };
        let (diff_days, _) = get_date_differences(&America, cfg.to_date_time());
        println!("{:?}", diff_days);

        let t = Rewards::get_rail_pass_jades(&cfg.rail_pass, diff_days);
        println!("{:?}", t);
    }
}
