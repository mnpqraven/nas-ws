use nas_ws::{handler::FromAxumResponse, routes::honkai::banner::gacha_banner_list};
use serde_json::json;
use std::collections::HashMap;
use tracing::info;
use url::Url;
use vercel_runtime::{http::bad_request, run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    run(handler).await
}
pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let parsed_url = Url::parse(&req.uri().to_string()).unwrap();
    let hash_query: HashMap<String, String> = parsed_url.query_pairs().into_owned().collect();
    let id_key = hash_query.get("id");

    info!("hash query: {:?}", hash_query);
    info!(id_key);

    match id_key {
        None => {
            return bad_request(json!( {
                "message": "Query string is invalid",
            }));
        }
        Some(id) => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(Body::Text(id.to_owned()))?),
    }
}
