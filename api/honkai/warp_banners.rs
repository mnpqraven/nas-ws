use nas_ws::{handler::FromAxumResponse, routes::honkai::banner::warp_banner_list};
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
pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    warp_banner_list().await.as_axum()
}
