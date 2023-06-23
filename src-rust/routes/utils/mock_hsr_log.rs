use crate::routes::{
    endpoint_types::List,
    utils::{Body, FromAxumResponse, WorkerError},
};
use axum::Json;
use fake::{Dummy, Fake, Faker};
use response_derive::JsonResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vercel_runtime::{Response, StatusCode};

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema, Dummy)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    event_name: Event,
    event_index: u32,
    fooo: String,
    bar: u32,
    bazz: f32,
    abc: String,
    sss: String,
    children: Vec<ChildLog>,
}
#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema, Dummy)]
#[serde(rename_all = "camelCase")]
pub struct ChildLog {
    event_name: Event,
    event_index: u32,
    fooo: String,
    bar: u32,
    bazz: f32,
    abc: String,
    sss: String,
}
#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema, Dummy)]
pub enum Event {
    TurnReset,
    TurnEnd,
    SPChange,
}

pub async fn handle() -> Result<Json<List<Log>>, WorkerError> {
    let mut t: Vec<Log> = vec![];
    for _ in 0..200 {
        let data = Faker.fake::<Log>();
        t.push(data);
    }
    Ok(Json(t.into()))
}
