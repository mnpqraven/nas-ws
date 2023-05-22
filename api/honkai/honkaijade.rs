use axum::{extract::FromRequest, http::Method, Json};
use nas_ws::handler::error::WorkerError;
use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    // if *req.method() != Method::POST {
    //     return Ok(WorkerError::WrongMethod.into());
    // }
    // let payload = Json::from_request(req, &()).await;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "message": "你好，世界"
            })
            .to_string()
            .into(),
        )?)
}
