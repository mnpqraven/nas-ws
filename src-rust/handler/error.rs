use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::fmt::Display;

/// general error type of the application, this should be used in junctions
/// between api handler methods (axum, vercel runtime etc.)
/// Within the application, for deep/small functions it is okay to use
/// `anyhow::Error` then convert to `WorkerError` by using
/// `Into<WorkerError>`
#[derive(Debug)]
pub enum WorkerError {
    ParseData(String),
    Computation(ComputationType),
    WrongMethod,
    EmptyBody,
    Unknown(anyhow::Error),
}

#[derive(Serialize, Debug, Clone)]
pub enum ComputationType {
    BadDateComparison,
    BadNumberCast,
}

impl WorkerError {
    pub fn code(&self) -> StatusCode {
        match self {
            WorkerError::ParseData(_) => StatusCode::BAD_REQUEST,
            WorkerError::Computation(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WorkerError::EmptyBody => StatusCode::BAD_REQUEST,
            WorkerError::WrongMethod => StatusCode::METHOD_NOT_ALLOWED,
            WorkerError::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl Display for WorkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = match self {
            Self::ParseData(reason) => format!("Incorrect Data\nReason: {}", reason),
            Self::Computation(comp_type) => match comp_type {
                ComputationType::BadDateComparison => {
                    "Bad date comparison, past date cannot be greater future date".to_owned()
                }
                // ComputationType::BadNumberCast => todo!(),
                _ => "Computation error from the server".to_owned(),
            },
            Self::WrongMethod => "Method is not supported".to_owned(),
            Self::EmptyBody => "Missing body data".to_owned(),
            Self::Unknown(err) => format!("Unknown error: {}", err),
        };
        write!(f, "{}", fmt)
    }
}

impl IntoResponse for WorkerError {
    fn into_response(self) -> Response {
        let code = self.code();
        let msg = self.to_string();
        if let WorkerError::Unknown(inner) = self {
            tracing::error!("stacktrace: {}", &inner.backtrace());
            inner.chain().for_each(|er| {
                tracing::error!("chain: {}", er);
            });
        } else {
            tracing::error!(msg);
        };
        (code, msg).into_response()
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

impl From<WorkerError> for vercel_runtime::Error {
    fn from(value: WorkerError) -> Self {
        vercel_runtime::Error::from(value.to_string())
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for WorkerError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::Unknown(err.into())
    }
}
