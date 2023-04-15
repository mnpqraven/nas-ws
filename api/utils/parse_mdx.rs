use axum::Json;
use nas_ws::{
    handler::error::WorkerError,
    routes::utils::{parse_mdx, MdxPayload},
};
use serde::Serialize;
use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[derive(Serialize)]
pub struct APIError {
    pub message: &'static str,
    pub code: &'static str,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    // check for POST
    let payload = match req.body() {
        // TODO:
        Body::Empty => todo!(),
        Body::Text(_) => todo!(),
        Body::Binary(data) => String::from_utf8(data.to_owned()).unwrap(),
    };
    let body = parse_mdx(Json(serde_json::from_str::<MdxPayload>(&payload)?)).await;
    match body {
        Ok(decoded_data) => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(decoded_data.into())?),
        Err(err) => Ok(Response::builder()
            .status(err.code())
            .body(err.to_string().into())?),
    }
}
