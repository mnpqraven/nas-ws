use self::types::{EstimateCfg, JadeEstimateResponse, RewardFrequency, RewardSource};
use crate::handler::error::WorkerError;
use axum::{extract::rejection::JsonRejection, Json};
use chrono::Utc;
use tracing::{error, instrument};

#[cfg(test)]
mod tests;
pub mod types;
pub mod rpc;

#[instrument(ret, err)]
pub async fn handle(
    rpayload: Result<Json<EstimateCfg>, JsonRejection>,
) -> Result<Json<JadeEstimateResponse>, WorkerError> {
    if let Ok(Json(payload)) = rpayload {
        let rewards = RewardSource::compile_sources(&payload)?;
        // let (diff_days, _) = get_date_differences(&payload.server, payload.get_until_date());
        let diff_days = RewardFrequency::Daily.get_difference(
            Utc::now(),
            payload.get_until_date(),
            &payload.server,
        )?;

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
