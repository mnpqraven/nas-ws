use crate::{
    handler::{
        error::{ComputationType, WorkerError},
        FromAxumResponse,
    },
    routes::honkai::{patch::types::Patch, utils::helpers::get_next_monday},
};
use axum::Json;
use chrono::{DateTime, Datelike, Duration, TimeZone, Utc, Weekday};
use num_derive::{FromPrimitive, ToPrimitive};
use prost::Enumeration;
use response_derive::JsonResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::debug;
use vercel_runtime::{Body, Response, StatusCode};

#[derive(Debug, Deserialize, Clone, JsonSchema)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct EstimateCfg {
    pub server: Server,
    pub until_date: SimpleDate,
    pub rail_pass: RailPassCfg,
    pub battle_pass: BattlePassOption,
    pub eq: EqTier,
    pub moc: u32,
    pub moc_current_week_done: bool,
    pub current_rolls: Option<i32>,
    pub current_jades: Option<i32>,
    pub daily_refills: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, JsonResponse, Clone, Default)]
pub struct JadeEstimateResponse {
    pub days: i32,
    pub rolls: i32,
    pub total_jades: i32,
    pub sources: Vec<RewardSource>,
}

#[derive(Deserialize, Clone, Debug, JsonSchema)]
pub struct SimpleDate {
    pub day: u32,
    pub month: u32,
    pub year: u32,
}

#[derive(Deserialize, Clone, Debug, JsonSchema, FromPrimitive, ToPrimitive, Enumeration)]
#[repr(i32)]
pub enum EqTier {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
}

impl EqTier {
    #[allow(dead_code)]
    fn from_level(level: u32) -> Result<EqTier, WorkerError> {
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

#[derive(
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    JsonResponse,
    JsonSchema,
    Clone,
    Copy,
    Enumeration,
    FromPrimitive,
)]
#[repr(i32)]
pub enum Server {
    Asia = 0,
    America = 1,
    Europe = 2,
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

#[derive(Debug, Serialize, Deserialize, JsonResponse, Clone)]
pub struct RewardSource {
    pub source: String,
    pub jades_amount: Option<i32>,
    pub rolls_amount: Option<i32>,
    pub source_type: RewardFrequency,
    pub description: Option<String>,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    JsonResponse,
    Clone,
    Copy,
    Enumeration,
)]
#[repr(i32)]
pub enum RewardFrequency {
    Daily = 0,
    Weekly = 1,
    BiWeekly = 2,
    Monthly = 3,
    WholePatch = 4,
    HalfPatch = 5,
    OneTime = 6,
}

impl RewardFrequency {
    pub fn get_difference(
        &self,
        from_date: DateTime<Utc>,
        to_date: DateTime<Utc>,
        server: &Server,
    ) -> Result<u32, WorkerError> {
        let from_date = today_at_reset(&from_date, server);

        // TODO: unit test all of these
        match self {
            RewardFrequency::Daily => Self::get_day_diff(from_date, to_date),
            RewardFrequency::Weekly => Self::get_week_diff(from_date, to_date, server),
            RewardFrequency::BiWeekly => Self::get_biweek_diff(from_date, to_date, server),
            RewardFrequency::Monthly => Self::get_month_diff(from_date, to_date),
            RewardFrequency::HalfPatch => Patch::half_patch_passed_diff(from_date, to_date),
            RewardFrequency::WholePatch => Patch::patch_passed_diff(from_date, to_date),
            RewardFrequency::OneTime => Ok(1),
        }
    }

    /// Counts number of days that has passed between 2 dates
    fn get_day_diff(from_date: DateTime<Utc>, to_date: DateTime<Utc>) -> Result<u32, WorkerError> {
        if from_date > to_date {
            return Err(WorkerError::Computation(ComputationType::BadDateComparison));
        }
        let mut diff_days = 0;
        DateRange(from_date, to_date).for_each(|_| {
            diff_days += 1;
        });
        Ok(diff_days)
    }

    /// Counts number of weeks that has passed between 2 dates
    fn get_week_diff(
        from_date: DateTime<Utc>,
        to_date: DateTime<Utc>,
        server: &Server,
    ) -> Result<u32, WorkerError> {
        if from_date > to_date {
            return Err(WorkerError::Computation(ComputationType::BadDateComparison));
        }
        let mut diff_weeks = 0;
        // padding to include monday
        DateRange(
            today_right_after_reset(&from_date, server),
            to_date + Duration::days(1),
        )
        .for_each(|date| {
            if date.weekday() == Weekday::Mon {
                diff_weeks += 1;
            }
        });
        Ok(diff_weeks)
    }

    /// Counts number of biweekly reset (MoC etc.) that has passed between 2
    /// dates
    /// NOTE: this is counted from the start of the game's launch, not from
    /// `from_date`
    fn get_biweek_diff(
        from_date: DateTime<Utc>,
        to_date: DateTime<Utc>,
        server: &Server,
    ) -> Result<u32, WorkerError> {
        if from_date > to_date {
            return Err(WorkerError::Computation(ComputationType::BadDateComparison));
        }
        let base_moc = Utc
            .with_ymd_and_hms(2023, 5, 29, Server::get_utc_reset_hour(server), 0, 0)
            .unwrap();

        let mut start = base_moc;
        while start < from_date {
            start += Duration::weeks(2);
        }
        let mut end = base_moc;
        while end < to_date {
            if end + Duration::weeks(2) > to_date {
                break;
            }
            end += Duration::weeks(2);
        }

        let mut diff_biweeks = 0;
        // this flag is to filter out every other monday that's not the start
        // of a new moc cycle
        let mut moc_start = true;
        for day in DateRange(start, end + Duration::days(1)) {
            if day.weekday() == Weekday::Mon {
                if moc_start {
                    diff_biweeks += 1;
                }
                moc_start = !moc_start;
            }
        }
        Ok(diff_biweeks)
    }

    pub fn get_month_diff(
        from_date: DateTime<Utc>,
        to_date: DateTime<Utc>,
    ) -> Result<u32, WorkerError> {
        if from_date > to_date {
            return Err(WorkerError::Computation(ComputationType::BadDateComparison));
        }
        let mut amount = 0;
        // padding 1 for first day of the month
        for date in DateRange(from_date, to_date + Duration::days(1)) {
            if let 1 = date.day() {
                amount += 1;
            }
        }
        Ok(amount)
    }
}

#[derive(Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct RailPassCfg {
    pub use_rail_pass: bool,
    pub days_left: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, JsonResponse, Clone, Copy, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BattlePassOption {
    pub battle_pass_type: BattlePassType,
    pub current_level: u32,
}
#[derive(
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    JsonResponse,
    JsonSchema,
    Clone,
    Copy,
    Enumeration,
    FromPrimitive,
)]
#[repr(i32)]
pub enum BattlePassType {
    None = 0,
    Basic = 1,
    Premium = 2,
}

pub fn today_right_after_reset(a: &DateTime<Utc>, server: &Server) -> DateTime<Utc> {
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
}

/// Get the reset date time
///
/// This will always rewind and never return a reset in the future
pub fn today_at_reset(a: &DateTime<Utc>, server: &Server) -> DateTime<Utc> {
    let mut res = Utc
        .with_ymd_and_hms(
            a.year(),
            a.month(),
            a.day(),
            server.get_utc_reset_hour(),
            0,
            0,
        )
        .unwrap();
    // can't forward, has to rewind by 1 day
    if res > *a {
        res -= Duration::days(1);
    }
    res
}

impl RewardSource {
    pub fn compile_sources(cfg: &EstimateCfg) -> Result<Vec<Self>, WorkerError> {
        let dt_to = cfg.get_until_date();
        let diff_days = RewardFrequency::Daily.get_difference(Utc::now(), dt_to, &cfg.server)?;

        let src_su = Self::src_su(&cfg.eq, &cfg.server, dt_to)?;
        let src_bp = Self::src_bp(cfg.battle_pass, dt_to, &cfg.server);
        let src_rail_pass = Self::src_rail_pass(&cfg.rail_pass, diff_days);
        let src_daily_mission = Self::src_daily_mission(diff_days);
        let src_daily_text = Self::src_daily_text(diff_days);
        let src_hoyolab_checkin = Self::src_hoyolab_checkin(dt_to);
        let src_moc = Self::src_moc(cfg.moc, cfg.moc_current_week_done, dt_to, &cfg.server)?;
        let src_char_trial = Self::src_char_trial(dt_to, &cfg.server)?;
        let src_ember_trade = Self::src_ember_trade(dt_to, &cfg.server)?;
        let mut sources = vec![
            src_su,
            src_bp,
            src_rail_pass,
            src_daily_mission,
            src_daily_text,
            src_hoyolab_checkin,
            src_moc,
            src_char_trial,
            src_ember_trade,
        ];
        if let Some(refills) = cfg.daily_refills {
            if refills != 0 {
                sources.push(Self::src_daily_refill(diff_days, refills));
            }
        }

        Ok(sources)
    }
    fn src_daily_refill(days: u32, amount: u32) -> Self {
        let refill_cost: i32 = match amount as i32 {
            1 => -50,
            2 | 3 => -75,
            4 | 5 => -100,
            6 | 7 => -150,
            8 => -200,
            _ => 0,
        };
        Self {
            source: "Daily Refills".into(),
            jades_amount: Some(refill_cost * days as i32),
            rolls_amount: None,
            source_type: RewardFrequency::Daily,
            description: None,
        }
    }

    fn src_bp(bp_config: BattlePassOption, dt_to: DateTime<Utc>, server: &Server) -> Self {
        let mut current_level = bp_config.current_level;
        let mut current_patch = Patch::current();
        // distribute rewards for the first monday check, avoid infinite
        let mut max_level_reached = bp_config.current_level >= 50; // default
        let start = get_next_monday(Utc::now(), server);

        let (mut rolls, mut jades) = (0, 0);
        for date in DateRange(start, dt_to + Duration::days(1)) {
            if date.weekday() == Weekday::Mon {
                // raise the bp level
                current_level = match current_level {
                    0..=40 => current_level + 10,
                    _ => 50,
                };
                // give bp rewards based on levels
                match current_level {
                    50 => {
                        if !max_level_reached {
                            jades += 680;
                        }
                        max_level_reached = true;
                    }
                    40..=49 => (), // self-molding resin instead of rolls
                    30..=39 => rolls += 2,
                    20..=29 => rolls += 1,
                    10..=19 => rolls += 1,
                    _ => (),
                };
            }
            // date crossed over to next patch
            if !current_patch.contains(date) {
                // iterate patch tracker
                current_patch.next();
                // reset the bp status
                current_level = 0;
                // distribute 1st time buying reward
                match bp_config.battle_pass_type {
                    BattlePassType::None => (),
                    BattlePassType::Basic => {
                        jades += 680;
                        rolls += 4
                    }
                    BattlePassType::Premium => {
                        jades += 880;
                        rolls += 4;
                        current_level += 10;
                    }
                }
            }
        }
        // prototypeShouldEnd
        let (final_jade, final_roll) = match bp_config.battle_pass_type {
            BattlePassType::None => (None, None), // f2p doesn't get any jade nor purchase rewards
            _ => (Some(jades), Some(rolls)),
        };
        Self {
            source: "Nameless Honor".into(),
            jades_amount: final_jade,
            rolls_amount: final_roll,
            source_type: RewardFrequency::WholePatch,
            description: None,
        }
    }

    fn src_su(
        eq_tier: &EqTier,
        server: &Server,
        until_date: DateTime<Utc>,
    ) -> Result<Self, WorkerError> {
        let per_weeks = match eq_tier {
            EqTier::Zero | EqTier::One => 75,
            EqTier::Two => 105,
            EqTier::Three => 135,
            EqTier::Four => 165,
            EqTier::Five => 195,
            // WARN: NEEDS CONFIRM
            EqTier::Six => 225,
        };
        let weeks = RewardFrequency::Weekly.get_difference(Utc::now(), until_date, server)?;
        Ok(Self {
            source: "Simulated Universe".into(),
            jades_amount: Some((weeks * per_weeks).try_into().unwrap()),
            rolls_amount: None,
            source_type: RewardFrequency::Weekly,
            description: None,
        })
    }

    fn src_daily_mission(days: u32) -> Self {
        let jades = (60 * days).try_into().unwrap();
        Self {
            source: "Daily missions".into(),
            jades_amount: Some(jades),
            rolls_amount: None,
            source_type: RewardFrequency::Daily,
            description: None,
        }
    }

    fn src_daily_text(days: u32) -> Self {
        let jades = (days * 5).try_into().unwrap();
        Self {
            source: "Daily text messages".into(),
            jades_amount: Some(jades),
            rolls_amount: None,
            source_type: RewardFrequency::Daily,
            description: Some("These text messeages are limited, you can run out of messages and you might get less in-game.".into())
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
            source_type: RewardFrequency::Monthly,
            description: None,
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
            source_type: RewardFrequency::Monthly,
            description: Some(
                "20 jades are distributed at the 5th, 13th and 20th every month.".into(),
            ),
        }
    }

    fn src_char_trial(until_date: DateTime<Utc>, server: &Server) -> Result<Self, WorkerError> {
        let freq = RewardFrequency::HalfPatch;
        let amount = freq.get_difference(Utc::now(), until_date, server)? as i32;
        Ok(Self {
            source: "Character Trials".into(),
            jades_amount: Some(20 * amount),
            rolls_amount: None,
            source_type: RewardFrequency::HalfPatch,
            description: None,
        })
    }

    fn src_ember_trade(until_date: DateTime<Utc>, server: &Server) -> Result<Self, WorkerError> {
        let freq = RewardFrequency::Monthly;
        let amount = 5 * freq.get_difference(Utc::now(), until_date, server)? as i32;
        Ok(Self {
            source: "Monthly ember exchange".into(),
            jades_amount: None,
            rolls_amount: Some(amount),
            source_type: freq,
            description: None,
        })
    }

    fn src_moc(
        stars: u32,
        // defaults to true
        done_this_week: bool,
        until_date: DateTime<Utc>,
        server: &Server,
    ) -> Result<Self, WorkerError> {
        let freq = RewardFrequency::BiWeekly;
        let mut diffs = freq.get_difference(Utc::now(), until_date, server)?;
        if !done_this_week {
            diffs += 1;
        }

        let amount: i32 = ((stars / 3) * 60 * diffs).try_into().unwrap();
        debug!(diffs, amount);
        Ok(Self {
            source: "Memory of chaos".into(),
            jades_amount: Some(amount),
            rolls_amount: None,
            source_type: RewardFrequency::BiWeekly,
            description: None,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct Pull {
    pub draw_number: u32,
    pub rate: f32,
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
    pub fn get_until_date(&self) -> DateTime<Utc> {
        let SimpleDate { day, month, year } = self.until_date;
        match self.server {
            Server::Asia => Utc.with_ymd_and_hms(year as i32, month, day, 19, 0, 0),
            Server::America => Utc.with_ymd_and_hms(year as i32, month, day, 9, 0, 0),
            Server::Europe => Utc.with_ymd_and_hms(year as i32, month, day, 12, 0, 0),
        }
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};

    use super::{RewardFrequency, Server};

    #[test]
    fn biweek() {
        let from = Utc::now();
        let diff_biweeks =
            RewardFrequency::get_biweek_diff(from, from + Duration::days(39), &Server::America)
                .unwrap();
        println!("{}", diff_biweeks);
    }
}
