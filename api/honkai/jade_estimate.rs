use axum::{extract::FromRequest, http::Method, Json};
use nas_ws::{
    handler::{error::WorkerError, FromAxumResponse},
    routes::honkai::jade_estimate::handle,
};
use vercel_runtime::{run, Body, Error, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false)
        .pretty()
        .init();
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    if *req.method() != Method::POST {
        return Ok(WorkerError::WrongMethod.into());
    }

    let payload = Json::from_request(req, &()).await;
    handle(payload).await.as_axum()
}
