use nas_ws::{handler::FromAxumResponse, routes::honkai::banner::gacha_banner_list};
use vercel_runtime::{Body, Error, Request, Response};

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    gacha_banner_list().await.as_axum()
}
