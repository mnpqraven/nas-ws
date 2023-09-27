use self::{
    cron::write_db, dotfiles::dotfiles_routes, health::health_check, honkai::honkai_routes,
    rpc_routes::rpc_routes, utils::utils_routes,
};
use axum::{routing::get, Router};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub mod cron;
pub mod dotfiles;
pub mod endpoint_types;
mod health;
pub mod honkai;
pub mod rpc_routes;
pub mod utils;

pub fn app_router() -> Router {
    Router::new()
        .nest("/utils", utils_routes())
        .nest("/dotfiles", dotfiles_routes())
        .nest("/honkai", honkai_routes())
        .nest("/cron", cron_routes())
        .nest("/", rpc_routes())
        .route("/health", get(health_check).post(health_check))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}

fn cron_routes() -> Router {
    Router::new().route("/write_db", get(write_db::execute))
}
