use axum::Json;
use nas_ws::{
    handler::{error::WorkerError, FromAxumResponse},
    routes::honkai::patch::{list_future_patch_date, types::Patch},
};
use response_derive::JsonResponse;
use serde::Serialize;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[derive(Serialize, JsonResponse, Clone, Debug)]
pub struct PatchList {
    pub patches: Vec<Patch>,
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_ansi(false)
        .init();
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    // let payload = Json::from_request(req, &()).await;
    let Json(data) = list_future_patch_date().await?;
    Ok(Json(PatchList { patches: data })).as_axum()
}
