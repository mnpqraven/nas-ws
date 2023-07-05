use crate::{
    handler::{error::WorkerError, FromAxumResponse},
    routes::honkai::{
        dm_api::{types::EquipmentConfig, write_big_trace},
        mhy_api::internal::{
            categorizing::{DbCharacter, DbCharacterSkill, DbCharacterSkillTree},
            impls::DbData,
        },
    },
};
use axum::Json;
use response_derive::JsonResponse;
use serde::Serialize;
use tracing::info;
use vercel_runtime::{Body, Response, StatusCode};

#[derive(Serialize, JsonResponse, Debug)]
pub struct CronResult {
    pub character_db: bool,
    pub skill_db: bool,
    pub trace_db: bool,
    pub big_trace_db: bool,
    pub lc_db: bool,
}

pub async fn write_db() -> Result<Json<CronResult>, WorkerError> {
    info!("write_db ...");
    let char_db = <DbCharacter as DbData<DbCharacter>>::try_write_disk().await;
    let skill_db = <DbCharacterSkill as DbData<DbCharacterSkill>>::try_write_disk().await;
    let trace_db = <DbCharacterSkillTree as DbData<DbCharacterSkillTree>>::try_write_disk().await;

    let big_trace_db = write_big_trace().await;
    let lc_db = <EquipmentConfig as DbData<EquipmentConfig>>::try_write_disk().await;

    Ok(Json(CronResult {
        character_db: char_db.is_ok(),
        skill_db: skill_db.is_ok(),
        trace_db: trace_db.is_ok(),
        big_trace_db: big_trace_db.is_ok(),
        lc_db: lc_db.is_ok(),
    }))
}
