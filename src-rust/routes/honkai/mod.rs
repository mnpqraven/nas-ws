pub mod constants;
pub mod gacha;
pub mod jade_estimate;
pub mod types;

use axum::{routing::get, Router};

use self::{gacha::probability_rate, jade_estimate::jade_estimate};

pub fn honkai_routes() -> Router {
    Router::new()
        .route(
            "/jade_estimate",
            get(jade_estimate).post(jade_estimate),
        )
        .route(
            "/probability_rate",
            get(probability_rate).post(probability_rate),
        )
}
