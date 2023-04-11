use crate::builder::MyParams;
use axum::{extract::Path, routing::get, Json, Router};
use serde::Deserialize;

pub fn foo_routes() -> Router {
    Router::new()
        .route("/foo", get(foo_get).post(foo_post))
        .route("/foo/test", get(foo_test_get).post(foo_test_post))
        .route("/foo/:id/:name", get(foo_id_name_get))
}

#[derive(Deserialize)]
struct UnknownPayload {
    data: String,
}

async fn foo_test_get() {
    // TODO:
    //
}
async fn foo_test_post() {
    // TODO:
}

async fn foo_post(Json(payload): Json<UnknownPayload>) {
    println!("hello {}", payload.data);
}

async fn foo_get() {}

async fn foo_id_name_get(Path(params): Path<MyParams>) {
    println!("hello id {} with name {}", params.id, params.name);
}
