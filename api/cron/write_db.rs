use axum::Json;
use nas_ws::handler::{error::WorkerError, FromAxumResponse};
use nas_ws::routes::honkai::mhy_api::internal::{
    categorizing::DbCharacter, constants::CHARACTER_LOCAL, impls::DbData,
};
use response_derive::JsonResponse;
use serde::Serialize;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[derive(Debug, Clone, Serialize, JsonResponse)]
struct ResponseData {
    character_db: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let char_db = <DbCharacter as DbData<DbCharacter>>::try_write_disk(CHARACTER_LOCAL)
        .await
        .is_ok();

    Ok(Json(ResponseData {
        character_db: char_db,
    }))
    .as_axum()
}
