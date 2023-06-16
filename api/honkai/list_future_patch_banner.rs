use axum::Json;
use nas_ws::{
    handler::{error::WorkerError, FromAxumResponse},
    routes::honkai::patch::{list_future_patch_banner, types::PatchBanner},
};
use response_derive::JsonResponse;
use serde::Serialize;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[derive(Serialize, JsonResponse, Clone, Debug)]
pub struct BannerList {
    banners: Vec<PatchBanner>,
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
    let Json(data) = list_future_patch_banner().await?;
    Ok(Json(BannerList { banners: data })).as_axum()
}
