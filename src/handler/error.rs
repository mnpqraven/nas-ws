use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::fmt::Display;

#[derive(Serialize, Debug)]
pub enum WorkerError {
    // reason text
    ParseData(String),
    Computation,
}

impl WorkerError {
    pub fn code(&self) -> StatusCode {
        match self {
            WorkerError::ParseData(_) => StatusCode::BAD_REQUEST,
            WorkerError::Computation => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
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

// use serde::ser::StdError;
impl From<WorkerError> for vercel_runtime::Error {
    fn from(value: WorkerError) -> Self {
        Self::from(value.to_string())
    }
}

impl IntoResponse for WorkerError {
    fn into_response(self) -> Response {
        (self.code(), self.to_string()).into_response()
    }
}
