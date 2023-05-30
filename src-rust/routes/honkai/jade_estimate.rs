use crate::handler::{error::WorkerError, FromAxumResponse};
use axum::{extract::rejection::JsonRejection, Json};
use chrono::{TimeZone, Utc};
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
    pub value: i32,
}

impl RewardSource {
    fn new(source: impl Into<String>, value: i32) -> Self {
        Self {
            source: source.into(),
            value,
        }
    }
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
        let dt_now = Utc::now();
        let diff = dt_to - dt_now;

        let diff_days = diff.num_days();
        let diff_weeks = match diff_days / 7 > diff.num_weeks() {
            true => diff.num_weeks() + 1,
            false => diff.num_weeks(),
        };

        let rewards: Vec<RewardSource> = vec![
            RewardSource::new(
                "Simulated Universe",
                Self::get_su_jades(cfg.level, diff_weeks),
            ),
            RewardSource::new("Battle Pass", Self::get_bp_jades(cfg.battle_pass)),
            RewardSource::new(
                "Rail Pass",
                Self::get_rail_pass_jades(cfg.rail_pass, diff_days as u32),
            ),
            RewardSource::new(
                "Dailies",
                Self::get_dailies_jades(cfg.level, diff_days as u32),
            ),
        ];

        let total_jades: i32 = rewards.iter().map(|e| e.value).sum();

        let rolls = match cfg.current_rolls {
            Some(rolls) => (total_jades / 160) + rolls,
            None => match cfg.current_jades {
                Some(jades) => (total_jades + jades) / 160,
                None => total_jades / 160,
            },
        };

        Self {
            total_jades,
            rolls,
            days: diff_days,
            sources: rewards,
        }
    }

    fn get_dailies_jades(level: u32, days: u32) -> i32 {
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
