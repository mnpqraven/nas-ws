use super::{
    constants::{
        HARD_DRAW, HARD_PITY_RATE, NORMAL_DRAW_RIGHT, NORMAL_RATE, SOFT_DRAW_LEFT, SOFT_DRAW_RIGHT,
        SOFT_PITY_RATE,
    },
    types::GachaCfg,
};
use crate::handler::error::WorkerError;
use axum::{extract::rejection::JsonRejection, Json};
use rand::{distributions::Bernoulli, prelude::Distribution};
use serde::{Deserialize, Serialize};

pub async fn gacha_cfg() -> Result<Json<GachaCfg>, WorkerError> {
    Ok(Json(GachaCfg::default()))
}

#[derive(Debug, Serialize, Deserialize)]
struct DistributedRate {
    draw_number: u32,
    percent: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rolls {
    pub rolls: u32,
}

pub async fn probability_rate(
    payload: Result<Json<Rolls>, JsonRejection>,
) -> Result<Json<GachaCfg>, WorkerError> {
    match payload {
        Ok(Json(rolls)) => {
            let mut rolls_since_last_ssr: u32 = 1;
            for pull in 1..=rolls.rolls {
                let rolled_ssr = roll(pull, rolls_since_last_ssr);
                rolls_since_last_ssr = match rolled_ssr {
                    true => 1,
                    false => rolls_since_last_ssr + 1,
                }
            }
            Ok(Json(GachaCfg::default()))
        }
        Err(err) => Err(WorkerError::ParseData(err.body_text())),
    }
}

fn roll(roll_number: u32, rolls_since_last_ssr: u32) -> bool {
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
    roll_result
}
