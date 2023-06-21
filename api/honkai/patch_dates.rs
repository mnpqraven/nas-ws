use nas_ws::{handler::FromAxumResponse, routes::honkai::banner::patch_date_list};
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
    // let payload = Json::from_request(req, &()).await;
    let data = patch_date_list().await?;
    Ok(data).as_axum()
}
