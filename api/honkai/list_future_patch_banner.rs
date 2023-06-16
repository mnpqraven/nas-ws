use axum::Json;
use nas_ws::{
    handler::FromAxumResponse,
    routes::{endpoint_types::List, honkai::patch::list_future_patch_banner},
};
use vercel_runtime::{run, Body, Error, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_ansi(false)
        .init();
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let data = List::new(list_future_patch_banner().await?.to_vec());
    Ok(Json(data)).as_axum()
}