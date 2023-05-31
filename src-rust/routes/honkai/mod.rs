pub mod constants;
pub mod gacha;
pub mod jade_estimate;
pub mod types;
pub mod utils;

use axum::{routing::get, Router};

use self::{
    gacha::probability_rate, jade_estimate::jade_estimate,
    utils::patch_date::list_future_patch_date,
};

pub fn honkai_routes() -> Router {
    Router::new()
        .route("/jade_estimate", get(jade_estimate).post(jade_estimate))
        .route(
            "/probability_rate",
            get(probability_rate).post(probability_rate),
        )
        .route("/list_future_patch_date", get(list_future_patch_date))
}
