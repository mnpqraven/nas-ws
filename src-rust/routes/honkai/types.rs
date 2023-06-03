use crate::handler::error::WorkerError;
use crate::handler::FromAxumResponse;
use axum::Json;
use chrono::{DateTime, Datelike, Duration, TimeZone, Utc};
use response_derive::JsonResponse;
use serde::{Deserialize, Serialize};
use std::path::Path;
use vercel_runtime::{Body, Response, StatusCode};

use super::jade_estimate::{get_current_patch_boundaries, get_date_differences};

#[derive(Serialize, Deserialize, JsonResponse, Clone)]
pub struct Rewards {
    pub sources: Vec<RewardSource>,
    pub total_jades: i32,
    pub rolls: i32,
    pub days: i64,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct EstimateCfg {
    pub until_date: SimpleDate,
    pub rail_pass: RailPassCfg,
    pub battle_pass: bool,
    pub eq: EqTier,
    pub current_rolls: Option<i32>,
    pub current_jades: Option<i32>,
}

#[derive(Serialize, Deserialize, JsonResponse, Clone)]
pub struct RewardSource {
    pub source: String,
    pub jades_amount: Option<i32>,
    pub rolls_amount: Option<i32>,
    pub source_type: RewardSourceType,
}

#[derive(Serialize, Deserialize, JsonResponse, Clone, Copy)]
pub enum RewardSourceType {
    Daily,
    Weekly,
    Monthly,
    WholePatch,
    HalfPatch,
    OneTime,
}

impl RewardSource {
    fn new_jade(
        source: impl Into<String>,
        calc_fn: impl Fn() -> i32,
        source_type: RewardSourceType,
    ) -> Self {
        Self {
            source: source.into(),
            jades_amount: Some(calc_fn()),
            rolls_amount: None,
            source_type,
        }
    }

    fn new_roll(
        source: impl Into<String>,
        calc_fn: impl Fn() -> i32,
        source_type: RewardSourceType,
    ) -> Self {
        Self {
            source: source.into(),
            jades_amount: None,
            rolls_amount: Some(calc_fn()),
            source_type,
        }
    }

    fn compile_sources(cfg: &EstimateCfg) -> Vec<Self> {
        let dt_to = cfg.until_date.to_date_time();
        let (diff_days, diff_weeks) = get_date_differences(None, dt_to);

        let daily = RewardSourceType::Daily;
        let weekly = RewardSourceType::Weekly;
        let monthly = RewardSourceType::Monthly;
        let patch_long = RewardSourceType::WholePatch;
        let patch_half = RewardSourceType::HalfPatch;

        let su = || Rewards::get_su_jades(&cfg.eq, diff_weeks);
        let bp = || Rewards::get_bp_jades(cfg.battle_pass);
        let rail_pass = || Rewards::get_rail_pass_jades(&cfg.rail_pass, diff_days);
        let daily_mission = || Rewards::get_daily_missions(&cfg.eq, diff_days);
        let daily_text = || Rewards::get_daily_text(diff_days);
        let lab_checkin = || Rewards::get_checkin_jades(dt_to);
        let char_trial = || {
            Rewards::get_character_trial_jades(get_current_patch_boundaries(Utc::now()).0, dt_to)
        };
        let ember_trade = || Rewards::get_monthly_ember_rolls(dt_to);

        vec![
            Self::new_jade("Simulated Universe", su, weekly),
            Self::new_jade("Battle Pass", bp, patch_long),
            Self::new_jade("Rail Pass", rail_pass, monthly),
            Self::new_jade("Daily missions", daily_mission, daily),
            Self::new_jade("Daily text messages", daily_text, daily),
            Self::new_jade("HoyoLab Check-in", lab_checkin, monthly),
            Self::new_jade("Character Trials", char_trial, patch_half),
            Self::new_roll("Monthly ember exchange", ember_trade, monthly),
        ]
    }
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
impl SimpleDate {
    fn to_date_time(&self) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(self.year as i32, self.month, self.day, 19, 0, 0)
            .unwrap()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub enum EqTier {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl EqTier {
    #[allow(dead_code)]
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

#[derive(Debug, Deserialize)]
pub struct Pull {
    pub draw_number: u32,
    pub rate: f32,
}

#[derive(Debug)]
pub struct SurveyRate(pub Vec<Pull>);

impl Default for SurveyRate {
    fn default() -> Self {
        let mut gacha_rate = SurveyRate(vec![]);
        let path = match true {
            true if Path::new("../assets/rate.csv").exists() => "../assets/rate.csv",
            true if Path::new("assets/rate.csv").exists() => "assets/rate.csv",
            _ => {
                panic!("assets not found, run `cargo run --bin tasks`");
            }
        };
        match csv::Reader::from_path(path) {
            Ok(mut rdr) => {
                for pull in rdr.deserialize::<Pull>().flatten() {
                    gacha_rate.0.push(pull);
                }
            }
            Err(_) => {
                panic!("assets not found, run `cargo run --bin tasks`");
            }
        }
        gacha_rate
    }
}

impl Rewards {
    pub fn from_cfg(cfg: EstimateCfg) -> Self {
        let rewards = RewardSource::compile_sources(&cfg);
        let (diff_days, _) = get_date_differences(None, cfg.until_date.to_date_time());

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

    fn get_daily_missions(eq_tier: &EqTier, days: u32) -> i32 {
        let daily = match eq_tier {
            // NOT CONFIRMED
            EqTier::Zero => 50,
            EqTier::One => 50,
            EqTier::Two => 50,
            EqTier::Three => 50,
            // NOTE: CONFIRMED
            EqTier::Four => 60,
            // NOT CONFIRMED
            EqTier::Five => 60,
            EqTier::Six => 60,
        };
        (daily * days).try_into().unwrap()
    }

    fn get_daily_text(days: u32) -> i32 {
        (days * 5).try_into().unwrap()
    }

    fn get_su_jades(eq_tier: &EqTier, weeks: i64) -> i32 {
        let per_weeks = match eq_tier {
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

    fn get_rail_pass_jades(cfg: &RailPassCfg, diff_days: u32) -> i32 {
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
        match patch_start + Duration::weeks(3) < until_date {
            true => 40,
            false => 20,
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

struct DateRange(DateTime<Utc>, DateTime<Utc>);
impl Iterator for DateRange {
    type Item = DateTime<Utc>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 < self.1 {
            let next = self.0 + Duration::days(1);
            Some(std::mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}
