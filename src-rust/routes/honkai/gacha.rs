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
use tracing::error;
use vercel_runtime::{Body, Response, StatusCode};

pub async fn gacha_cfg() -> Result<Json<GachaCfg>, WorkerError> {
    Ok(Json(GachaCfg::default()))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DistributedRate {
    draw_number: u32,
    percent: f64,
    eidolon: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ProbabilityRatePayload {
    pub rolls: u32,
    pub next_guaranteed: bool,
    pub simulate_result: bool,
}

#[derive(Debug, Serialize, JsonResponse, Clone)]
pub struct ProbabilityRateResponse {
    rolls: u32,
    rates: Vec<DistributedRate>,
}

pub async fn probability_rate(
    rpayload: Result<Json<ProbabilityRatePayload>, JsonRejection>,
) -> Result<Json<ProbabilityRateResponse>, WorkerError> {
    let mut rates: Vec<DistributedRate> = vec![];
    if let Ok(Json(payload)) = rpayload {
        let mut rolls_since_last_ssr: u32 = 1;
        let mut accumulated_rate: f64 = 1.0 - 0.006;
        let mut eidolon_count: u32 = 0;

        for pull in 1..=payload.rolls {
            let rolled_ssr = roll(pull, rolls_since_last_ssr, payload.simulate_result);

            update_rate(&mut accumulated_rate, rolls_since_last_ssr);

            rolls_since_last_ssr = match rolled_ssr {
                true => 1,
                false => rolls_since_last_ssr + 1,
            };

            if rolled_ssr {
                eidolon_count += 1;
            }

            rates.push(DistributedRate {
                draw_number: pull,
                percent: 1.0 - accumulated_rate,
                eidolon: eidolon_count,
            });
        }
        Ok(Json(ProbabilityRateResponse {
            rolls: payload.rolls,
            rates,
        }))
    } else {
        let err = rpayload.unwrap_err();
        error!("{}", err.body_text());
        Err(WorkerError::ParseData(err.body_text()))
    }
}

fn update_rate(accumulated_rate: &mut f64, last_ssr: u32) {
    let percent = get_percent(last_ssr);
    if last_ssr == 1 {
        *accumulated_rate = 1.0;
    }
    *accumulated_rate *= 1.0 - percent;
}

fn get_percent(last_ssr: u32) -> f64 {
    match last_ssr {
        1..=NORMAL_DRAW_RIGHT => NORMAL_RATE,
        SOFT_DRAW_LEFT..=SOFT_DRAW_RIGHT => SOFT_PITY_RATE,
        HARD_DRAW => HARD_PITY_RATE,
        // NOTE: should never hit
        _ => 0.0,
    }
}

fn roll(roll_number: u32, rolls_since_last_ssr: u32, simulate_result: bool) -> bool {
    let percent = get_percent(rolls_since_last_ssr);

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
