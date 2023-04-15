use std::fmt::Display;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum WorkerError {
    // reason text
    ParseData(String),
    Computation,
}

impl Display for WorkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = match self {
            WorkerError::ParseData(reason) => format!("Incorrect Data\nReason: {}", reason),
            WorkerError::Computation => "Computation error from the server".to_owned(),
        };
        write!(f, "{}", fmt)
    }
}

impl IntoResponse for WorkerError {
    fn into_response(self) -> Response {
        let code: StatusCode = match self {
            Self::ParseData(_) => StatusCode::BAD_REQUEST,
            Self::Computation => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (code, self.to_string()).into_response()
    }
}
