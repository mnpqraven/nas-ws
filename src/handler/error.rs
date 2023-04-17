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
    WrongMethod,
    EmptyBody,
}

impl WorkerError {
    pub fn code(&self) -> StatusCode {
        match self {
            WorkerError::ParseData(_) => StatusCode::BAD_REQUEST,
            WorkerError::Computation => StatusCode::INTERNAL_SERVER_ERROR,
            WorkerError::EmptyBody => StatusCode::BAD_REQUEST,
            WorkerError::WrongMethod => StatusCode::METHOD_NOT_ALLOWED,
        }
    }
}

impl Display for WorkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = match self {
            Self::ParseData(reason) => format!("Incorrect Data\nReason: {}", reason),
            Self::Computation => "Computation error from the server".to_owned(),
            Self::WrongMethod => "Method is not supported".to_owned(),
            Self::EmptyBody => "Missing body data".to_owned(),
        };
        write!(f, "{}", fmt)
    }
}

impl IntoResponse for WorkerError {
    fn into_response(self) -> Response {
        (self.code(), self.to_string()).into_response()
    }
}

impl From<WorkerError> for vercel_runtime::Response<vercel_runtime::Body> {
    fn from(value: WorkerError) -> Self {
        // is this a safe unwrap ?
        vercel_runtime::Response::builder()
            .status(value.code())
            .body(value.to_string().into())
            .unwrap()
    }
}
