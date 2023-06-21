use axum::extract::Path;
use nas_ws::{handler::FromAxumResponse, routes::honkai::mhy_api::internal::character_by_id};
use serde_json::json;
use std::collections::HashMap;
use url::Url;
use vercel_runtime::{http::bad_request, run, Body, Error, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false)
        .pretty()
        .init();
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let parsed_url = Url::parse(&req.uri().to_string()).unwrap();
    let hash_query: HashMap<String, String> = parsed_url.query_pairs().into_owned().collect();
    let id_key = hash_query.get("id1");

    match id_key {
        None => bad_request(json!({
            "message": "Query string is invalid",
        })),
        Some(id) => {
            let id: u32 = id.parse()?;
            Ok(character_by_id(Path(id)).await?).as_axum()
        }
    }
}
