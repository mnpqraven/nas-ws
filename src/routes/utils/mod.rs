use crate::{handler::error::WorkerError, routes::utils::parse_mdx::parse_mdx};
use axum::{routing::post, Router};
use response_derive::OthiResponse;
use serde::{Deserialize, Serialize};
use vercel_runtime::{Body, Response, StatusCode};
use crate::handler::FromAxumResponse;

pub mod parse_mdx;

#[derive(Deserialize)]
pub struct MdxPayload {
    #[serde(rename = "fileData")]
    pub file_data: String,
}

#[derive(Serialize, Deserialize, OthiResponse)]
pub struct DecodedDataForm {
    pub title: String,
    pub description: String,
    pub content: String,
}

pub fn utils_routes() -> Router {
    Router::new().route("/parse_mdx", post(parse_mdx))
}
