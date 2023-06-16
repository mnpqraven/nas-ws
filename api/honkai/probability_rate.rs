use axum::{extract::FromRequest, Json};
use nas_ws::{handler::FromAxumResponse, routes::honkai::probability_rate::handle};
use vercel_runtime::{Body, Error, Request, Response};

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let payload = Json::from_request(req, &()).await;
    handle(payload).await.as_axum()
}
