use axum::{extract::FromRequest, Json};
use nas_ws::{handler::FromAxumResponse, routes::honkai::probability_rate::handle};
use vercel_runtime::{run, Body, Error, Request, Response};

/// INFO: https://www.reddit.com/r/Genshin_Impact/comments/kdy1ky/everyone_is_misunderstanding_soft_pity/
pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let payload = Json::from_request(req, &()).await;
    handle(payload).await.as_axum()
}
