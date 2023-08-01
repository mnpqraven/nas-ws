use axum::Json;
use tonic::{Request, Response, Status};

use crate::handler::error::WorkerError;

use self::jadeestimate::jade_estimate_service_server::JadeEstimateService;
use super::types::{EstimateCfg, JadeEstimateResponse, RewardSource};

pub mod jadeestimate {
    tonic::include_proto!("jadeestimate");
}

#[tonic::async_trait]
impl JadeEstimateService for JadeEstimateResponse {
    async fn post(
        &self,
        req: Request<jadeestimate::JadeEstimateCfg>,
    ) -> Result<Response<jadeestimate::JadeEstimateResponse>, Status> {
        let payload: EstimateCfg = req.into_inner().try_into()?;
        let Json(data) = super::handle(Ok(Json(payload))).await?;

        Ok(Response::new(data.into()))
    }
}

impl From<JadeEstimateResponse> for jadeestimate::JadeEstimateResponse {
    fn from(value: JadeEstimateResponse) -> Self {
        jadeestimate::JadeEstimateResponse {
            days: value.days,
            rolls: value.rolls,
            total_jades: value.total_jades,
            sources: value.sources.into_iter().map(|e| e.into()).collect(),
        }
    }
}

impl TryFrom<jadeestimate::JadeEstimateCfg> for EstimateCfg {
    type Error = WorkerError;

    fn try_from(value: jadeestimate::JadeEstimateCfg) -> Result<Self, WorkerError> {
        let jadeestimate::SimpleDate { day, month, year } =
            value.until_date.ok_or(WorkerError::EmptyBody)?;

        let jadeestimate::RailPassCfg {
            days_left,
            use_rail_pass,
        } = value.rail_pass.ok_or(WorkerError::EmptyBody)?;

        let jadeestimate::BattlePassOption {
            battle_pass_type,
            current_level,
        } = value.battle_pass.ok_or(WorkerError::EmptyBody)?;

        let data = EstimateCfg {
            server: super::types::Server::from_i32(value.server).unwrap(),
            until_date: super::types::SimpleDate { day, month, year },
            rail_pass: super::types::RailPassCfg {
                use_rail_pass,
                days_left,
            },
            battle_pass: super::types::BattlePassOption {
                battle_pass_type: super::types::BattlePassType::from_i32(battle_pass_type).unwrap(),
                current_level,
            },
            eq: super::types::EqTier::from_i32(value.eq).unwrap(),
            moc: value.moc,
            moc_current_week_done: value.moc_current_week_done,
            current_rolls: value.current_rolls,
            current_jades: value.current_jades,
            daily_refills: value.daily_refills,
        };
        Ok(data)
    }
}

impl From<RewardSource> for jadeestimate::RewardSource {
    fn from(value: RewardSource) -> Self {
        Self {
            source: value.source,
            source_type: value.source_type as i32,
            jades_amount: value.jades_amount,
            rolls_amount: value.rolls_amount,
            description: value.description,
        }
    }
}
