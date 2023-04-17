use axum::{extract::FromRequest, http::Method, Json};
use nas_ws::handler::{error::WorkerError, FromAxumResponse};
use nas_ws::routes::utils::parse_mdx::parse_mdx;
use vercel_runtime::{run, Body, Error, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    if *req.method() != Method::POST {
        return Ok(WorkerError::WrongMethod.into());
    }
    let payload = Json::from_request(req, &()).await;
    parse_mdx(payload).await.as_axum()
}
