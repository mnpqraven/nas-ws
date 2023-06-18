use axum::Json;
use nas_ws::{
    handler::FromAxumResponse,
    routes::{endpoint_types::List, honkai::patch::list_future_patch_date},
};
use vercel_runtime::{run, Body, Error, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    run(handler).await
}
pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    // let payload = Json::from_request(req, &()).await;
    let data = List::new(list_future_patch_date().await?.to_vec());
    Ok(Json(data)).as_axum()
}
