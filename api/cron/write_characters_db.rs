use std::{collections::HashMap, fs, path::Path};

use axum::{extract::FromRequest, http::Method, Json};
use nas_ws::routes::honkai::mhy_api::{internal::get_character_list};
use nas_ws::routes::honkai::mhy_api::internal::categorizing::Character;
use serde_json::json;
use tracing::debug;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_ansi(false)
        .init();
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    // NOTE: uncomment if payload is used (will be eventually)
    let res_str: String = reqwest::get(
        "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_new/en/characters.json",
    )
    .await?
    .text()
    .await?;
    let map: HashMap<String, Character> = serde_json::from_str(&res_str)?;
    let characters = map.into_values().collect::<Vec<Character>>();

    let exist_status = match Path::new("/tmp/characters.json").exists() {
        true => "exist",
        false => "not exist",
    };
    let write_attempt = fs::write(
        "/tmp/characters.json",
        serde_json::to_vec_pretty(&characters)?,
    );
    let write_status = match write_attempt {
        Ok(_) => "write OK",
        Err(_) => "write ERROR",
    };
    debug!("exist_status: {}", exist_status);
    debug!("write_status: {}", write_status);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        // WARN: caching for browsers only, don't debug on postman
        .body(
            json!({ "exist": exist_status, "write_status": write_status })
                .to_string()
                .into(),
        )?)
}
