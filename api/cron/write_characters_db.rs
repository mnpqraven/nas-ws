use axum::Json;
use nas_ws::handler::{error::WorkerError, FromAxumResponse};
use nas_ws::routes::honkai::mhy_api::internal::write_character_db;
use response_derive::JsonResponse;
use serde::Serialize;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[derive(Debug, Clone, Serialize, JsonResponse)]
struct ResponseData {
    exist_status: bool,
    write_status: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let (exist_status, write_status) = write_character_db().await?;

    Ok(Json(ResponseData {
        exist_status,
        write_status,
    }))
    .as_axum()
}
