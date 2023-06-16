use axum::Json;
use nas_ws::{
    handler::FromAxumResponse,
    routes::{endpoint_types::List, honkai::patch::list_future_patch_banner},
};
use vercel_runtime::{Body, Error, Request, Response};

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let data = List::new(list_future_patch_banner().await?.to_vec());
    Ok(Json(data)).as_axum()
}
