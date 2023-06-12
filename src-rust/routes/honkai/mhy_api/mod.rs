use self::types::MihoResponse;
use crate::handler::error::WorkerError;
use axum::{extract::rejection::JsonRejection, Json};
use serde::Deserialize;
use tracing::error;

pub mod types;

// NOTE: mihoyo public api + assets
// https://api.mihomo.me/sr_info/700164437
// https://api.mihomo.me/sr_info_parsed/700164437?lang=en
// https://github.com/Mar-7th/StarRailRes

#[derive(Debug, Deserialize)]
pub struct Payload {
    id: u32,
}
pub async fn handle(
    rpayload: Result<Json<Payload>, JsonRejection>,
) -> Result<Json<MihoResponse>, WorkerError> {
    if rpayload.is_err() {
        let err = rpayload.unwrap_err();
        error!("{}", err.body_text());
        return Err(WorkerError::ParseData(err.body_text()));
    }
    // safe unwrap
    let Json(Payload { id }) = rpayload.unwrap();

    let url = format!("https://api.mihomo.me/sr_info_parsed/{id}?lang=en");
    let res = reqwest::get(url)
        .await
        .map_err(|e| WorkerError::ParseData(format!("failed request {e}")))?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res
        .json::<MihoResponse>()
        .await
        .map_err(|e| WorkerError::ParseData(format!("failed parsing data {e}")))?;
    println!("{}", serde_json::to_string_pretty(&body).unwrap());

    Ok(Json(body))
}
