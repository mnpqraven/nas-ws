use super::{
    constants::{
        HARD_DRAW, HARD_PITY_RATE, NORMAL_DRAW_RIGHT, NORMAL_RATE, SOFT_DRAW_LEFT, SOFT_DRAW_RIGHT,
        SOFT_PITY_RATE,
    },
    types::GachaCfg,
};
use crate::handler::{error::WorkerError, FromAxumResponse};
use axum::{extract::rejection::JsonRejection, Json};
use rand::{distributions::Bernoulli, prelude::Distribution};
use response_derive::JsonResponse;
use serde::{Deserialize, Serialize};
use tracing::debug;
use vercel_runtime::{Body, Response, StatusCode};

pub async fn gacha_cfg() -> Result<Json<GachaCfg>, WorkerError> {
    Ok(Json(GachaCfg::default()))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DistributedRate {
    draw_number: u32,
    percent: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rolls {
    pub rolls: u32,
}

#[derive(Debug, Serialize, Deserialize, JsonResponse, Clone)]
pub struct ProbabilityRateResponse {
    rolls: u32,
    rates: Vec<DistributedRate>,
}

pub async fn probability_rate(
    payload: Result<Json<Rolls>, JsonRejection>,
) -> Result<Json<ProbabilityRateResponse>, WorkerError> {
    let mut rates: Vec<DistributedRate> = vec![];

    match payload {
        Ok(Json(rolls)) => {
            let mut rolls_since_last_ssr: u32 = 1;
            let mut accumulated_rate: f64 = (1.0 - 0.006);
            for pull in 1..=rolls.rolls {
                let rolled_ssr = roll(pull, rolls_since_last_ssr, false);

                mutate_rate(&mut accumulated_rate, rolls_since_last_ssr);

                rolls_since_last_ssr = match rolled_ssr {
                    true => 1,
                    false => rolls_since_last_ssr + 1,
                };
                rates.push(DistributedRate {
                    draw_number: pull,
                    percent: 1.0 - accumulated_rate,
                });
            }
            Ok(Json(ProbabilityRateResponse {
                rolls: rolls.rolls,
                rates,
            }))
        }
        Err(err) => Err(WorkerError::ParseData(err.body_text())),
    }
}

fn mutate_rate(accumulated_rate: &mut f64, last_ssr: u32) {
    let percent = match last_ssr {
        1..=NORMAL_DRAW_RIGHT => NORMAL_RATE,
        SOFT_DRAW_LEFT..=SOFT_DRAW_RIGHT => SOFT_PITY_RATE,
        HARD_DRAW => HARD_PITY_RATE,
        // NOTE: should never hit
        _ => 0.0,
    };

    if last_ssr == 1 {
        *accumulated_rate = 1.0;
    }
    // let next_rate_miss: f64 = *accumulated_rate * percent;
    *accumulated_rate *= (1.0 - percent);
    debug!(
        "last: ({}): * {} = {}",
        last_ssr,
        (1.0 - percent),
        accumulated_rate
    )
}

fn roll(roll_number: u32, rolls_since_last_ssr: u32, simulate_result: bool) -> bool {
    // will decide the rate

    let percent = match rolls_since_last_ssr {
        1..=NORMAL_DRAW_RIGHT => NORMAL_RATE,
        SOFT_DRAW_LEFT..=SOFT_DRAW_RIGHT => SOFT_PITY_RATE,
        HARD_DRAW => HARD_PITY_RATE,
        // NOTE: should never hit
        _ => 0.0,
    };

    let roll_result = Bernoulli::new(percent)
        .unwrap()
        .sample(&mut rand::thread_rng());
    println!(
        "roll {}: {:?}, rate: {} %, since last: {}",
        roll_number,
        roll_result,
        percent * 100.0,
        rolls_since_last_ssr,
    );
    match simulate_result {
        false if rolls_since_last_ssr == HARD_DRAW => true,
        false => false,
        true => roll_result,
    }
}
