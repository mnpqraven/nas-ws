use super::constants::{
    HARD_DRAW, HARD_PITY_RATE, NORMAL_DRAW_RIGHT, NORMAL_RATE, SOFT_DRAW_LEFT, SOFT_DRAW_RIGHT,
    SOFT_PITY_RATE,
};
use crate::handler::{error::WorkerError, FromAxumResponse};
use axum::{extract::rejection::JsonRejection, Json};
use rand::{distributions::Bernoulli, prelude::Distribution};
use response_derive::JsonResponse;
use serde::{Deserialize, Serialize};
use tracing::{debug, error};
use vercel_runtime::{Body, Response, StatusCode};

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ProbabilityRatePayload {
    pub rolls: u32,
    pub next_guaranteed: bool,
    pub simulate_result: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DistributedRate {
    pub uni_draw_number: u32,
    pub percent_eidolons: Vec<(f64, u32)>,
}

// master struct
#[derive(Debug, Serialize, JsonResponse, Clone)]
pub struct ProbabilityRateResponse {
    pub next_guaranteed: bool,
    pub roll_budget: u32,
    pub line: RateByPull,
}
impl ProbabilityRateResponse {
    fn new(roll_budget: u32, next_guaranteed: bool) -> Self {
        Self {
            next_guaranteed,
            roll_budget,
            line: Self::generate_line(roll_budget, next_guaranteed),
        }
    }

    fn generate_line(roll_budget: u32, next_guaranteed: bool) -> RateByPull {
        RateByPull::create_lines(roll_budget, next_guaranteed)
    }
}

#[derive(Debug, Default, Serialize, Clone, JsonResponse)]
pub struct RateByPull {
    pub data: Vec<DistributedRate>,
}
impl RateByPull {
    fn create_lines(rolls: u32, next_guaranteed: bool) -> Self {
        let mut rate_array: Vec<DistributedRate> = (1..=rolls)
            .into_iter()
            .map(|e| DistributedRate {
                uni_draw_number: e,
                percent_eidolons: vec![],
            })
            .collect();
        let mut tracker_0 = RollTracker::new(0);
        let mut tracker_1 = RollTracker::new(1);
        let mut tracker_2 = RollTracker::new(2);
        let mut tracker_3 = RollTracker::new(3);
        let mut tracker_4 = RollTracker::new(4);
        let mut tracker_5 = RollTracker::new(5);
        let mut tracker_6 = RollTracker::new(6);

        for (array_index, _pull_number) in (1..=rolls).into_iter().enumerate() {
            // we have E0 next ssr
            if next_guaranteed {
                // tracker 0 update
                tracker_0.accumulated_fail_rate *=
                    next_accumulated_fail_rate(tracker_0.rolls_since_last_ssr);
                tracker_0.rolls_since_last_ssr += 1;
                let actual_graph_rate = 1.0 - tracker_0.accumulated_fail_rate;

                let specific_rate = rate_array.get_mut(array_index).unwrap();
                specific_rate.percent_eidolons.push((actual_graph_rate, 0));
                println!("{}", actual_graph_rate);

                // debug!(tracker_0.rolls_since_last_ssr, actual_graph_rate);
            } else {
                if tracker_0.rolls_since_last_ssr == 1 {
                    tracker_0.accumulated_fail_rate *=
                        next_accumulated_fail_rate(tracker_0.rolls_since_last_ssr);
                } else {
                    tracker_0.accumulated_fail_rate *=
                        next_accumulated_fail_rate(tracker_0.rolls_since_last_ssr - 1)
                            * next_accumulated_fail_rate(tracker_0.rolls_since_last_ssr);
                }
                tracker_0.rolls_since_last_ssr += 1;
                let actual_graph_rate = 1.0 - tracker_0.accumulated_fail_rate;

                let specific_rate = rate_array.get_mut(array_index).unwrap();
                specific_rate.percent_eidolons.push((actual_graph_rate, 0));

                // debug!(tracker_0.rolls_since_last_ssr, actual_graph_rate);
            }
            // calc the rate of getting first ssr
        }
        // 1: rate(1)
        // 2: rate(2) * rate(1)
        // 3: rate(3) * rate(2)
        // ...
        // 90: rate(90) * rate(89) -> close 0.5
        // ...
        // 180: rate(180) * rate(179) -> close 1.0

        Self { data: rate_array }
    }

    fn get_rates(&self) -> Vec<DistributedRate> {
        self.data.clone()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Serialize, JsonResponse)]
pub struct RollTracker {
    pub rolls_since_last_ssr: u32,
    /// this will slowly goes to 0.0 at roll 90
    pub accumulated_fail_rate: f64,
    pub eidolon: u32,
}
impl RollTracker {
    fn new(eidolon: u32) -> Self {
        Self {
            rolls_since_last_ssr: 1,
            accumulated_fail_rate: 1.0 - 0.006,
            eidolon,
        }
    }
    fn get_attr(&self) -> (&u32, &f64, &u32) {
        (
            &self.rolls_since_last_ssr,
            &self.accumulated_fail_rate,
            &self.eidolon,
        )
    }
    fn get_mut_attr(&mut self) -> (&mut u32, &mut f64, &mut u32) {
        (
            &mut self.rolls_since_last_ssr,
            &mut self.accumulated_fail_rate,
            &mut self.eidolon,
        )
    }
}

pub async fn probability_rate(
    rpayload: Result<Json<ProbabilityRatePayload>, JsonRejection>,
) -> Result<Json<ProbabilityRateResponse>, WorkerError> {
    if rpayload.is_err() {
        let err = rpayload.unwrap_err();
        error!("{}", err.body_text());
        return Err(WorkerError::ParseData(err.body_text()));
    }
    // safe unwrap
    let Json(payload) = rpayload.unwrap();

    let master_prob_rate = ProbabilityRateResponse::new(payload.rolls, payload.next_guaranteed);

    Ok(Json(master_prob_rate))
}

fn next_accumulated_fail_rate(last_ssr: u32) -> f64 {
    // we just hit an SSR !
    match last_ssr == 1 {
        true => 1.0,
        false => 1.0 - get_ssr_percent(last_ssr),
    }
}

fn get_ssr_percent(last_ssr: u32) -> f64 {
    match last_ssr {
        1..=NORMAL_DRAW_RIGHT => NORMAL_RATE,
        SOFT_DRAW_LEFT..=SOFT_DRAW_RIGHT => SOFT_PITY_RATE,
        HARD_DRAW => HARD_PITY_RATE,
        // NOTE: should never hit
        _ => 0.0,
    }
}

fn roll_debug(roll_number: u32, rolls_since_last_ssr: u32, simulate_result: bool) -> bool {
    let percent = get_ssr_percent(rolls_since_last_ssr);

    let roll_result = Bernoulli::new(percent)
        .unwrap()
        .sample(&mut rand::thread_rng());
    println!(
        "roll {}: rate: {} %, since last: {}",
        roll_number,
        percent * 100.0,
        rolls_since_last_ssr,
    );
    match simulate_result {
        false if rolls_since_last_ssr == HARD_DRAW => true,
        false => false,
        true => roll_result,
    }
}
