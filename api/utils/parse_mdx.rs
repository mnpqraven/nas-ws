use axum::extract::FromRequest;
use axum::Json;
use nas_ws::handler::FromAxumResponse;
use nas_ws::routes::utils::{parse_mdx::parse_mdx, DecodedDataForm};
use vercel_runtime::{run, Body, Error, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let payload = Json::from_request(req, &()).await;
    DecodedDataForm::from_axum(parse_mdx(payload).await)
}
