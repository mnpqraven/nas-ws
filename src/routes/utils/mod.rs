pub mod parse_mdx;

use self::parse_mdx::Decoder;
use crate::{
    handler::{error::WorkerError, FromAxumResponse},
    routes::utils::parse_mdx::parse_mdx,
};
use axum::{routing::post, Json, Router};
use response_derive::JsonResponse;
use serde::{Deserialize, Serialize};
use vercel_runtime::{Body, Response, StatusCode};

#[derive(Deserialize)]
pub struct MdxPayload {
    #[serde(rename = "fileData")]
    pub file_data: String,
}

pub struct EncodedFile {
    pub filetype: String,
    pub decoder: Decoder,
    pub encoded_data: String,
}

#[derive(Serialize, Deserialize, JsonResponse, Clone)]
pub struct DecodedDataForm {
    pub title: String,
    pub description: String,
    pub content: String,
}

pub fn utils_routes() -> Router {
    Router::new().route("/parse_mdx", post(parse_mdx))
}
