use crate::handler::FromAxumResponse;
use crate::{handler::error::WorkerError, routes::utils::parse_mdx::parse_mdx};
use axum::{routing::post, Router};
use response_derive::JsonResponse;
use serde::{Deserialize, Serialize};
use vercel_runtime::{Body, Response, StatusCode};
use self::parse_mdx::Decoder;

pub mod parse_mdx;

#[derive(Deserialize)]
pub struct MdxPayload {
    #[serde(rename = "fileData")]
    pub file_data: String,
}

pub struct EncodedFile {
    pub filetype: String,
    pub decoder: Decoder,
    pub encoded_data: String
}

#[derive(Serialize, Deserialize, JsonResponse)]
pub struct DecodedDataForm {
    pub title: String,
    pub description: String,
    pub content: String,
}

pub fn utils_routes() -> Router {
    Router::new().route("/parse_mdx", post(parse_mdx))
}
