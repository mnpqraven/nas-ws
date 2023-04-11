use self::foo::foo_routes;
use axum::{routing::get, Router};

pub mod foo;

pub fn app_router() -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/foo", foo_routes())
}

async fn root() -> &'static str {
    "Hello, World!"
}
