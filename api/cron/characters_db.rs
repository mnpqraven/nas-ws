use nas_ws::routes::honkai::mhy_api::internal::get_character_list;
use serde_json::json;
use vercel_runtime::{Body, Error, Request, Response, StatusCode};

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    // NOTE: uncomment if payload is used (will be eventually)
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        // WARN: caching for browsers only, don't debug on postman
        .header("Cache-Control", "max-age=0, s-maxage=86400")
        .body(
            json!({ "characters": get_character_list().await.unwrap() })
                .to_string()
                .into(),
        )?)
}
