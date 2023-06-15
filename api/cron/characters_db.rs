use axum::{extract::FromRequest, http::Method, Json};
use nas_ws::routes::honkai::mhy_api::internal::get_character_list;
use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_ansi(false)
        .init();
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    // NOTE: uncomment if payload is used (will be eventually)
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Cache-Control", "max-age=0, s-maxage=86400")
        .body(
            json!({ "characters": get_character_list().await.unwrap() })
                .to_string()
                .into(),
        )?)
}
