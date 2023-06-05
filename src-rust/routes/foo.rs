use crate::builder::MyParams;
use axum::{extract::Path, routing::get, Json, Router};
use serde::Deserialize;

pub fn foo_routes() -> Router {
    Router::new()
        // INFO: /foo
        .route("/", get(foo_get).post(foo_post))
        .route("/:id/:name", get(foo_id_name_get))
}

#[derive(Deserialize)]
struct UnknownPayload {
    data: String,
}

async fn foo_post(Json(payload): Json<UnknownPayload>) -> String {
    format!("hello {}", payload.data)
}

async fn foo_get() {}

async fn foo_id_name_get(Path(params): Path<MyParams>) -> String {
    format!("hello id {} with name {}", params.id, params.name)
}
