use self::{
    cron::write_db, dotfiles::dotfiles_routes, foo::foo_routes, honkai::honkai_routes,
    utils::utils_routes,
};
use axum::{routing::get, Router};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub mod cron;
pub mod dotfiles;
pub mod endpoint_types;
mod foo;
pub mod honkai;
pub mod utils;

pub fn app_router() -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/utils", utils_routes())
        .nest("/foo", foo_routes())
        .nest("/dotfiles", dotfiles_routes())
        .nest("/honkai", honkai_routes())
        .nest("/cron", cron_routes())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}

async fn root() -> &'static str {
    "Hello, World!"
}

fn cron_routes() -> Router {
    Router::new().route("/write_db", get(write_db::write_db))
}
