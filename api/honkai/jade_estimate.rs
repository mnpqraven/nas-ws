use axum::{extract::FromRequest, http::Method, Json};
use nas_ws::{
    handler::{error::WorkerError, FromAxumResponse},
    routes::honkai::jade_estimate::handle,
};
use vercel_runtime::{run, Body, Error, Request, Response};

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    if *req.method() != Method::POST {
        return Ok(WorkerError::WrongMethod.into());
    }
    // NOTE: uncomment if payload is used (will be eventually)
    let payload = Json::from_request(req, &()).await;
    handle(payload).await.as_axum()
}
