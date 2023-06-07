use super::{
    jade_estimate::{get_current_patch_boundaries, get_date_differences},
    utils::patch_date::PatchList,
};
use crate::handler::error::WorkerError;
use crate::handler::FromAxumResponse;
use axum::Json;
use chrono::{DateTime, Datelike, Duration, TimeZone, Utc};
use response_derive::JsonResponse;
use serde::{Deserialize, Serialize};
use std::path::Path;
use vercel_runtime::{Body, Response, StatusCode};

#[derive(Serialize, Deserialize, JsonResponse, Clone)]
pub struct JadeEstimateResponse {
    pub sources: Vec<RewardSource>,
    pub total_jades: i32,
    pub rolls: i32,
    pub days: i64,
}

impl From<EstimateCfg> for JadeEstimateResponse {
    fn from(cfg: EstimateCfg) -> Self {
        let rewards = RewardSource::compile_sources(&cfg);
        let (diff_days, _) = get_date_differences(&cfg.server, cfg.to_date_time());

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
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct EstimateCfg {
    pub server: Server,
    pub until_date: SimpleDate,
    pub rail_pass: RailPassCfg,
    pub battle_pass: BattlePassOption,
    pub eq: EqTier,
    pub current_rolls: Option<i32>,
    pub current_jades: Option<i32>,
}
#[derive(Deserialize, Clone, Debug)]
pub enum Server {
    Asia,
    America,
    Europe,
}
impl Server {
    pub fn get_utc_reset_hour(&self) -> u32 {
        match self {
            Server::Asia => 19,
            Server::America => 9,
            Server::Europe => 12,
        }
    }
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

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum BattlePassOption {
    None,
    Basic,
    Premium,
}

impl RewardSource {
    fn compile_sources(cfg: &EstimateCfg) -> Vec<Self> {
        let dt_to = cfg.to_date_time();
        let (diff_days, _) = get_date_differences(&cfg.server, dt_to);

        let src_su = Self::src_su(&cfg.eq, dt_to);
        let src_bp = Self::src_bp(cfg.battle_pass, dt_to);
        let src_rail_pass = Self::src_rail_pass(&cfg.rail_pass, diff_days);
        let src_daily_mission = Self::src_daily_mission(diff_days);
        let src_daily_text = Self::src_daily_text(diff_days);
        let src_hoyolab_checkin = Self::src_hoyolab_checkin(dt_to);
        let src_char_trial =
            Self::src_char_trial(get_current_patch_boundaries(Utc::now()).0, dt_to);
        let src_ember_trade = Self::src_ember_trade(dt_to);

        vec![
            src_su,
            src_bp,
            src_rail_pass,
            src_daily_mission,
            src_daily_text,
            src_hoyolab_checkin,
            src_char_trial,
            src_ember_trade,
        ]
    }

    fn src_bp(bp_config: BattlePassOption, dt_to: DateTime<Utc>) -> Self {
        let patches = PatchList::patches_passed_number(dt_to) as i32;
        let (jades_amount, rolls_amount) = match bp_config {
            BattlePassOption::None => (None, None),
            BattlePassOption::Basic => (Some((680 + 680) * patches), None),
            BattlePassOption::Premium => (Some((880 + 680) * patches), Some(4 * patches)),
        };
        Self {
            source: "Battle Pass".into(),
            jades_amount,
            rolls_amount,
            source_type: RewardSourceType::Weekly,
        }
    }

    fn src_su(eq_tier: &EqTier, until_date: DateTime<Utc>) -> Self {
        let per_weeks = match eq_tier {
            EqTier::Zero | EqTier::One => 75,
            EqTier::Two => 105,
            EqTier::Three => 135,
            EqTier::Four => 165,
            EqTier::Five => 195,
            // WARN: NEEDS CONFIRM
            EqTier::Six => 225,
        };
        let mut amount = 0;
        for date in DateRange(Utc::now(), until_date) {
            if let chrono::Weekday::Mon = date.weekday() {
                amount += per_weeks;
            }
        }
        Self {
            source: "Simulated Universe".into(),
            jades_amount: Some(amount),
            rolls_amount: None,
            source_type: RewardSourceType::Weekly,
        }
    }

    fn src_daily_mission(days: u32) -> Self {
        let jades = (60 * days).try_into().unwrap();
        Self {
            source: "Daily missions".into(),
            jades_amount: Some(jades),
            rolls_amount: None,
            source_type: RewardSourceType::Daily,
        }
    }

    fn src_daily_text(days: u32) -> Self {
        let jades = (days * 5).try_into().unwrap();
        Self {
            source: "Daily text messages".into(),
            jades_amount: Some(jades),
            rolls_amount: None,
            source_type: RewardSourceType::Daily,
        }
    }

    pub fn src_rail_pass(cfg: &RailPassCfg, diff_days: u32) -> Self {
        let jades = match cfg.days_left {
            Some(days_left) if cfg.use_rail_pass => match days_left < diff_days {
                true => 90 * diff_days as i32 + 300 * diff_days as i32 / 30,
                false => 90 * diff_days as i32,
            },
            None if cfg.use_rail_pass => 90 * diff_days as i32,
            _ => 0,
        };
        Self {
            source: "Rail Pass".into(),
            jades_amount: Some(jades),
            rolls_amount: None,
            source_type: RewardSourceType::Monthly,
        }
    }

    fn src_hoyolab_checkin(until_date: DateTime<Utc>) -> Self {
        let mut amount: i32 = 0;
        for date in DateRange(Utc::now(), until_date) {
            if let 5 | 13 | 20 = date.day() {
                amount += 20;
            }
        }
        Self {
            source: "HoyoLab Check-in".into(),
            jades_amount: Some(amount),
            rolls_amount: None,
            source_type: RewardSourceType::Monthly,
        }
    }

    fn src_char_trial(patch_start: DateTime<Utc>, until_date: DateTime<Utc>) -> Self {
        // rewards from both banners in patch
        let jades = match patch_start + Duration::weeks(3) < until_date {
            true => 40,
            false => 20,
        };
        Self {
            source: "Character Trials".into(),
            jades_amount: Some(jades),
            rolls_amount: None,
            source_type: RewardSourceType::HalfPatch,
        }
    }

    fn src_ember_trade(until_date: DateTime<Utc>) -> Self {
        let mut amount: i32 = 0;
        for date in DateRange(Utc::now(), until_date) {
            if let 1 = date.day() {
                amount += 5;
            }
        }
        Self {
            source: "Monthly ember exchange".into(),
            jades_amount: None,
            rolls_amount: Some(amount),
            source_type: RewardSourceType::Monthly,
        }
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

impl JadeEstimateResponse {
    pub fn from_cfg(cfg: EstimateCfg) -> Self {
        let rewards = RewardSource::compile_sources(&cfg);
        let (diff_days, _) = get_date_differences(&cfg.server, cfg.to_date_time());

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
}

pub struct DateRange(pub DateTime<Utc>, pub DateTime<Utc>);
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

impl EstimateCfg {
    pub fn to_date_time(&self) -> DateTime<Utc> {
        let SimpleDate { day, month, year } = self.until_date;
        match self.server {
            Server::Asia => Utc.with_ymd_and_hms(year as i32, month, day, 19, 0, 0),
            Server::America => Utc.with_ymd_and_hms(year as i32, month, day, 9, 0, 0),
            Server::Europe => Utc.with_ymd_and_hms(year as i32, month, day, 12, 0, 0),
        }
        .unwrap()
    }
}
