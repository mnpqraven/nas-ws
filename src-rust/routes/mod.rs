use self::{
    dotfiles::dotfiles_routes, foo::foo_routes, honkai::honkai_routes, utils::utils_routes,
};
use axum::{http::Method, routing::get, Router};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

pub mod dotfiles;
mod foo;
pub mod honkai;
pub mod utils;

pub fn app_router() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT])
        .allow_origin(Any);

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
    "Hello, World!"
}
