use crate::{
    handler::{error::WorkerError, FromAxumResponse},
    routes::honkai::mhy_api::internal::{
        categorizing::{DbCharacter, DbCharacterSkill, DbCharacterSkillTree},
        constants::{CHARACTER_LOCAL, CHARACTER_SKILL_LOCAL, CHARACTER_SKILL_TREE_LOCAL},
        impls::DbData,
    },
};
use axum::Json;
use response_derive::JsonResponse;
use serde::Serialize;
use vercel_runtime::{Body, Response, StatusCode};

#[derive(Serialize, JsonResponse)]
pub struct CronResult {
    pub character_db: bool,
    pub skill_db: bool,
    pub trace_db: bool,
}

pub async fn write_db() -> Result<Json<CronResult>, WorkerError> {
    let char_db = <DbCharacter as DbData<DbCharacter>>::try_write_disk(CHARACTER_LOCAL).await;
    let skill_db =
        <DbCharacterSkill as DbData<DbCharacterSkill>>::try_write_disk(CHARACTER_SKILL_LOCAL).await;
    let trace_db = <DbCharacterSkillTree as DbData<DbCharacterSkillTree>>::try_write_disk(
        CHARACTER_SKILL_TREE_LOCAL,
    )
    .await;
    Ok(Json(CronResult {
        character_db: char_db.is_ok(),
        skill_db: skill_db.is_ok(),
        trace_db: trace_db.is_ok(),
    }))
}
