use self::{foo::foo_routes, utils::utils_routes, dotfiles::dotfiles_routes};
use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

pub mod dotfiles;
mod foo;
pub mod utils;

pub fn app_router() -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/utils", utils_routes())
        .nest("/foo", foo_routes())
        .nest("/dotfiles", dotfiles_routes())
        .layer(TraceLayer::new_for_http())
}

async fn root() -> &'static str {
    "Hello, World!"
}
