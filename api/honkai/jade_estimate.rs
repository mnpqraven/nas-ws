use axum::{extract::FromRequest, http::Method, Json};
use nas_ws::{
    handler::{error::WorkerError, FromAxumResponse},
    routes::honkai::jade_estimate::jade_estimate,
};
use vercel_runtime::{run, Body, Error, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    run(handler).await
}

/// INFO: https://www.reddit.com/r/Genshin_Impact/comments/kdy1ky/everyone_is_misunderstanding_soft_pity/
pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    if *req.method() != Method::POST {
        return Ok(WorkerError::WrongMethod.into());
    }
    // NOTE: uncomment if payload is used (will be eventually)
    let payload = Json::from_request(req, &()).await;
    jade_estimate(payload).await.as_axum()
}
