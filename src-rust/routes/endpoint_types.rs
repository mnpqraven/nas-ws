use crate::handler::{error::WorkerError, FromAxumResponse};
use axum::Json;
use serde::Serialize;
use vercel_runtime::{Body, Response, StatusCode};

#[derive(Serialize, Debug)]
pub struct List<T> {
    pub list: Vec<T>,
}

impl<T> From<Vec<T>> for List<T> {
    fn from(list: Vec<T>) -> Self {
        List { list }
    }
}

impl<T> List<T> {
    pub fn new(list: Vec<T>) -> Self {
        Self { list }
    }
}

impl<T: Serialize> FromAxumResponse<List<T>, WorkerError, vercel_runtime::Error>
    for Result<Json<List<T>>, WorkerError>
{
    type TFrom = Json<List<T>>;
    type TTo = Response<Body>;

    fn as_axum(&self) -> Result<Response<Body>, vercel_runtime::Error> {
        self.as_ref()
            .map_or_else(|err| Ok(err.clone().into()), |Json(val)| Ok(val.into()))
    }
}

impl<T: Serialize> From<&List<T>> for vercel_runtime::Response<Body> {
    fn from(value: &List<T>) -> Self {
        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body::<Body>(serde_json::to_string(value).unwrap().into())
            .unwrap()
    }
}

impl<T: Serialize> From<List<T>> for vercel_runtime::Response<Body> {
    fn from(value: List<T>) -> Self {
        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body::<Body>(serde_json::to_string(&value).unwrap().into())
            .unwrap()
    }
}
