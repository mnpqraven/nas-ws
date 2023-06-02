use std::mem;

use crate::handler::{error::WorkerError, FromAxumResponse};
use axum::{extract::rejection::JsonRejection, Json};
use chrono::{DateTime, Datelike, Duration, TimeZone, Utc};
use response_derive::JsonResponse;
use serde::{Deserialize, Serialize};
use tracing::error;
use vercel_runtime::{Body, Response, StatusCode};

// TODO: export as binding
#[derive(Serialize, Deserialize, JsonResponse, Clone)]
pub struct Rewards {
    pub sources: Vec<RewardSource>,
    pub total_jades: i32,
    pub rolls: i32,
    pub days: i64,
}

#[derive(Serialize, Deserialize, JsonResponse, Clone)]
pub struct RewardSource {
    pub source: String,
    pub jades_amount: Option<i32>,
    pub rolls_amount: Option<i32>,
    pub source_type: RewardSourceType,
}

#[derive(Serialize, Deserialize, JsonResponse, Clone)]
pub enum RewardSourceType {
    Daily,
    Weekly,
    Monthly,
    WholePatch,
    HalfPatch,
    OneTime,
}

impl RewardSource {
    fn new_jade(source: impl Into<String>, value: i32, source_type: RewardSourceType) -> Self {
        Self {
            source: source.into(),
            jades_amount: Some(value),
            rolls_amount: None,
            source_type,
        }
    }
    fn new_roll(source: impl Into<String>, value: i32, source_type: RewardSourceType) -> Self {
        Self {
            source: source.into(),
            jades_amount: None,
            rolls_amount: Some(value),
            source_type,
        }
    }
}

/// [TODO:description]
///
/// * `from_date`: [TODO:parameter]
/// * `to_date`: [TODO:parameter]
/// returns a tuple of differences in days and weeks, week diff is always rounded up (e.g a difference of 17-18 days would equate to 3 weeks)
fn get_date_differences(from_date: Option<DateTime<Utc>>, to_date: DateTime<Utc>) -> (u32, i64) {
    let dt_now = from_date.unwrap_or(Utc::now());
    let diff = to_date - dt_now;

    let diff_days = diff.num_days() as u32;
    let diff_weeks = match i64::from(diff_days / 7) > diff.num_weeks() {
        true => diff.num_weeks() + 1,
        false => diff.num_weeks(),
    };
    (diff_days, diff_weeks)
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct EstimateCfg {
    // INFO: ISO string
    pub until_date: SimpleDate,
    pub rail_pass: RailPassCfg,
    pub battle_pass: bool,
    pub level: u32,
    pub current_rolls: Option<i32>,
    pub current_jades: Option<i32>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct RailPassCfg {
    pub use_rail_pass: bool,
    pub days_left: Option<u32>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SimpleDate {
    pub day: u32,
    pub month: u32,
    pub year: u32,
}

enum EqTier {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl EqTier {
    fn from_level(level: i32) -> Result<EqTier, WorkerError> {
        match level {
            0..=19 => Ok(EqTier::Zero),
            20..=29 => Ok(EqTier::One),
            30..=39 => Ok(EqTier::Two),
            40..=49 => Ok(EqTier::Three),
            50..=59 => Ok(EqTier::Four),
            60..=64 => Ok(EqTier::Five),
            x if x >= 65 => Ok(EqTier::Six),
            _ => Err(WorkerError::ParseData(format!(
                "{} is not a valid level value",
                level
            ))),
        }
    }
}

impl Rewards {
    fn from_cfg(cfg: EstimateCfg) -> Self {
        let dt_to = Utc
            .with_ymd_and_hms(
                cfg.until_date.year as i32,
                cfg.until_date.month,
                cfg.until_date.day,
                19,
                0,
                0,
            )
            .unwrap();
        let (diff_days, diff_weeks) = get_date_differences(None, dt_to);

        // TODO: better weekly rewards algorithm
        // eval to see if iteraing rewards by days is cheap enough
        let rewards: Vec<RewardSource> = vec![
            RewardSource::new_jade(
                "Simulated Universe",
                Self::get_su_jades(cfg.level, diff_weeks),
                RewardSourceType::Weekly,
            ),
            RewardSource::new_jade(
                "Battle Pass",
                Self::get_bp_jades(cfg.battle_pass),
                RewardSourceType::WholePatch,
            ),
            RewardSource::new_jade(
                "Rail Pass",
                Self::get_rail_pass_jades(cfg.rail_pass, diff_days),
                RewardSourceType::Monthly,
            ),
            RewardSource::new_jade(
                "Daily missions",
                Self::get_daily_missions(cfg.level, diff_days),
                RewardSourceType::Daily,
            ),
            RewardSource::new_jade(
                "Daily text messages",
                Self::get_daily_text(diff_days),
                RewardSourceType::Daily,
            ),
            RewardSource::new_jade(
                "HoyoLab Check-in",
                Self::get_checkin_jades(dt_to),
                RewardSourceType::Monthly,
            ),
            RewardSource::new_jade(
                "Character Trials",
                Self::get_character_trial_jades(get_current_patch_boundaries(Utc::now()).0, dt_to),
                RewardSourceType::HalfPatch,
            ),
            RewardSource::new_roll(
                "Monthly ember exchange",
                Self::get_monthly_ember_rolls(dt_to),
                RewardSourceType::Monthly,
            ),
        ];

        let mut total_jades: i32 = rewards.iter().map(|e| e.jades_amount.unwrap_or(0)).sum();
        let reward_rolls: i32 = rewards.iter().map(|e| e.rolls_amount.unwrap_or(0)).sum();

        if let Some(current_jades) = cfg.current_jades {
            total_jades += current_jades;
        }
        let mut total_rolls = (total_jades / 160) + reward_rolls;
        if let Some(current_rolls) = cfg.current_rolls {
            total_rolls += current_rolls;
        }

        Self {
            total_jades,
            rolls: total_rolls,
            days: diff_days.try_into().unwrap(),
            sources: rewards,
        }
    }

    fn get_daily_missions(level: u32, days: u32) -> i32 {
        let daily = match EqTier::from_level(level as i32).unwrap() {
            EqTier::Zero => todo!(),
            EqTier::One => todo!(),
            EqTier::Two => todo!(),
            EqTier::Three => todo!(),
            // NOTE: CONFIRMED
            EqTier::Four => 60,
            EqTier::Five => todo!(),
            EqTier::Six => todo!(),
        };
        (daily * days).try_into().unwrap()
    }

    fn get_daily_text(days: u32) -> i32 {
        (days * 5).try_into().unwrap()
    }

    fn get_su_jades(level: u32, weeks: i64) -> i32 {
        let per_weeks = match EqTier::from_level(level as i32).unwrap() {
            // WARN: NEEDS CONFIRM
            EqTier::Zero => 60,
            EqTier::One => 75,
            // NOTE: CONFIRMED
            EqTier::Two => 105,
            EqTier::Three => 135,
            EqTier::Four => 165,
            // WARN: NEEDS CONFIRM
            EqTier::Five => 165,
            EqTier::Six => 165,
        };
        (per_weeks * weeks).try_into().unwrap()
    }

    fn get_bp_jades(use_bp: bool) -> i32 {
        match use_bp {
            true => 680,
            false => 0,
        }
    }

    fn get_rail_pass_jades(cfg: RailPassCfg, diff_days: u32) -> i32 {
        match cfg.days_left {
            Some(days_left) if cfg.use_rail_pass => match days_left < diff_days {
                true => 90 * diff_days as i32 + 300 * diff_days as i32 / 30,
                false => 90 * diff_days as i32,
            },
            None if cfg.use_rail_pass => 90 * diff_days as i32,
            _ => 0,
        }
    }

    fn get_checkin_jades(until_date: DateTime<Utc>) -> i32 {
        let mut amount: i32 = 0;
        for date in DateRange(Utc::now(), until_date) {
            if let 5 | 13 | 20 = date.day() {
                amount += 20;
            }
        }
        amount
    }

    fn get_character_trial_jades(patch_start: DateTime<Utc>, until_date: DateTime<Utc>) -> i32 {
        // rewards from both banners in patch
        if patch_start + Duration::weeks(3) < until_date {
            40
        } else {
            20
        }
    }

    fn get_monthly_ember_rolls(until_date: DateTime<Utc>) -> i32 {
        let mut amount: i32 = 0;
        for date in DateRange(Utc::now(), until_date) {
            if let 1 = date.day() {
                amount += 5;
            }
        }
        amount
    }
}

fn get_current_patch_boundaries(current_date: DateTime<Utc>) -> (DateTime<Utc>, DateTime<Utc>) {
    let base_1_1 = Utc.with_ymd_and_hms(2023, 6, 7, 2, 0, 0).unwrap();
    let (mut l_bound, mut r_bound) = (base_1_1, base_1_1 + Duration::weeks(6));
    while r_bound < current_date {
        l_bound = r_bound;
        r_bound += Duration::weeks(6);
    }
    (l_bound, r_bound)
}

struct DateRange(DateTime<Utc>, DateTime<Utc>);
impl Iterator for DateRange {
    type Item = DateTime<Utc>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 < self.1 {
            let next = self.0 + Duration::days(1);
            Some(mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}

pub async fn jade_estimate(
    rpayload: Result<Json<EstimateCfg>, JsonRejection>,
) -> Result<Json<Rewards>, WorkerError> {
    if let Ok(Json(payload)) = rpayload {
        Ok(Json(Rewards::from_cfg(payload)))
    } else {
        let err = rpayload.unwrap_err();
        error!("{}", err.body_text());
        Err(WorkerError::ParseData(err.body_text()))
    }
}
