use axum::{extract::FromRequest, Json};
use nas_ws::{handler::FromAxumResponse, routes::honkai::gacha::probability_rate};
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
    let payload = Json::from_request(req, &()).await;
    probability_rate(payload).await.as_axum()
}
