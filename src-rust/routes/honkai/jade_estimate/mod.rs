use self::types::{DateRange, EstimateCfg, JadeEstimateResponse, RewardSource, Server};
use crate::handler::error::WorkerError;
use axum::{extract::rejection::JsonRejection, Json};
use chrono::{DateTime, Datelike, Duration, TimeZone, Utc, Weekday};
use tracing::error;

pub mod types;

pub async fn jade_estimate(
    rpayload: Result<Json<EstimateCfg>, JsonRejection>,
) -> Result<Json<JadeEstimateResponse>, WorkerError> {
    if let Ok(Json(payload)) = rpayload {
        let rewards = RewardSource::compile_sources(&payload).unwrap();
        let (diff_days, _) = get_date_differences(&payload.server, payload.get_until_date());

        let mut total_jades: i32 = rewards.iter().map(|e| e.jades_amount.unwrap_or(0)).sum();
        let reward_rolls: i32 = rewards.iter().map(|e| e.rolls_amount.unwrap_or(0)).sum();

        if let Some(current_jades) = payload.current_jades {
            total_jades += current_jades;
        }
        let mut total_rolls = (total_jades / 160) + reward_rolls;
        if let Some(current_rolls) = payload.current_rolls {
            total_rolls += current_rolls;
        }

        let response = JadeEstimateResponse {
            total_jades,
            rolls: total_rolls,
            days: diff_days.try_into().unwrap(),
            sources: rewards,
        };

        Ok(Json(response))
    } else {
        let err = rpayload.unwrap_err();
        error!("{}", err.body_text());
        Err(WorkerError::ParseData(err.body_text()))
    }
}

/// Get a difference in days and weeks between 2 dates
///
/// WARN: TESTING IS NEEDED FOR WEEK DIFF
/// TODO: diffing test with RewardSourceType ::Daily.get_difference() and
/// safely remove this
///
/// returns a tuple of differences in days and weeks, week diff is always
/// rounded up (e.g a difference of 17-18 days would equate to 3 weeks)
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
