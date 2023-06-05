mod builder;
mod routes;
mod handler;

use crate::routes::app_router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let addr = SocketAddr::from(([0, 0, 0, 0], 5005));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app_router().into_make_service())
        .await
        .unwrap();
}
