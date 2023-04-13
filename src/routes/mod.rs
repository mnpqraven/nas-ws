use self::{foo::foo_routes, utils::utils_routes};
use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

pub mod foo;
pub mod utils;

pub fn app_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .nest("/utils", utils_routes())
        .nest("/foo", foo_routes())
        .layer(TraceLayer::new_for_http())
}

async fn root() -> &'static str {
    "Goodbye, World!"
}
async fn health() -> &'static str {
    match 1 == 2 {
        true => "All system green",
        false => "Something went wrong",
    }
}
