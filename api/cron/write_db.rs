use axum::Json;
use nas_ws::handler::{error::WorkerError, FromAxumResponse};
use nas_ws::routes::honkai::mhy_api::internal::categorizing::DbCharacterSkillTree;
use nas_ws::routes::honkai::mhy_api::internal::{
    categorizing::{DbCharacter, DbCharacterSkill},
    constants::*,
    impls::DbData,
};
use response_derive::JsonResponse;
use serde::Serialize;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[derive(Serialize, JsonResponse)]
struct ResponseData {
    character_db: bool,
    skill_db: bool,
    trace_db: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_ansi(false)
        .pretty()
        .init();
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let char_db = <DbCharacter as DbData<DbCharacter>>::try_write_disk(CHARACTER_LOCAL).await;
    let skill_db =
        <DbCharacterSkill as DbData<DbCharacterSkill>>::try_write_disk(CHARACTER_SKILL_LOCAL).await;
    let trace_db = <DbCharacterSkillTree as DbData<DbCharacterSkillTree>>::try_write_disk(
        CHARACTER_SKILL_TREE_LOCAL,
    )
    .await;

    Ok(Json(ResponseData {
        character_db: char_db.is_ok(),
        skill_db: skill_db.is_ok(),
        trace_db: trace_db.is_ok(),
    }))
    .as_axum()
}
