use self::foo::foo_routes;
use axum::{routing::get, Router};

pub mod foo;

pub fn app_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .nest("/foo", foo_routes())
}

async fn root() -> &'static str {
    "Goodbye, World!"
}
async fn health() -> &'static str {
    match 1 == 2 {
        true => "All system green",
        false => "Something went wrong"
    }
}
