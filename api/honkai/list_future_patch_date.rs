use nas_ws::{
    handler::FromAxumResponse, routes::honkai::utils::patch_date::list_future_patch_date,
};
use vercel_runtime::{run, Body, Error, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_ansi(false)
        .init();
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    // let payload = Json::from_request(req, &()).await;
    list_future_patch_date().await.as_axum()
}
