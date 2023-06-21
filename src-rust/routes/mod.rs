use self::{
    dotfiles::dotfiles_routes, foo::foo_routes, honkai::honkai_routes, utils::utils_routes,
};
use axum::{routing::get, Router};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub mod dotfiles;
mod foo;
pub mod honkai;
pub mod utils;
pub mod endpoint_types;

pub fn app_router() -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/utils", utils_routes())
        .nest("/foo", foo_routes())
        .nest("/dotfiles", dotfiles_routes())
        .nest("/honkai", honkai_routes())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}

async fn root() -> &'static str {
    "Goodbye, World!"
}
