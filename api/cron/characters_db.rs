use axum::{extract::FromRequest, http::Method, Json};
use nas_ws::{
    handler::error::WorkerError, routes::honkai::mhy_api::internal::write_character_list,
};
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

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    if *req.method() != Method::POST {
        return Ok(WorkerError::WrongMethod.into());
    }
    // NOTE: uncomment if payload is used (will be eventually)
    write_character_list()
        .await
        .map_err(|err| err.to_string())?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "status": "finished"
            })
            .to_string()
            .into(),
        )?)
}
