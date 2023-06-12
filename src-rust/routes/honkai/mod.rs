pub mod banner;
pub mod jade_estimate;
pub mod mhy_api;
pub mod patch;
pub mod probability_rate;
pub mod utils;

use self::{banner::gacha_banner_list, patch::list_future_patch_date};
use axum::routing::{get, post};
use axum::Router;

pub fn honkai_routes() -> Router {
    Router::new()
        .route(
            "/jade_estimate",
            get(jade_estimate::handle).post(jade_estimate::handle),
        )
        .route(
            "/probability_rate",
            get(probability_rate::handle).post(probability_rate::handle),
        )
        .route("/list_future_patch_date", get(list_future_patch_date))
        .route("/gacha_banner_list", get(gacha_banner_list))
        .route("/mhy", post(mhy_api::handle))
}
