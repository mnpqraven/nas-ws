use axum::Json;
use nas_ws::{
    handler::FromAxumResponse,
    routes::{endpoint_types::List, honkai::patch::list_future_patch_date},
};
use vercel_runtime::{Body, Error, Request, Response};

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    // let payload = Json::from_request(req, &()).await;
    let data = List::new(list_future_patch_date().await?.to_vec());
    Ok(Json(data)).as_axum()
}
