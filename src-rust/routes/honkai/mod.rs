pub mod banner;
// pub mod constants;
pub mod jade_estimate;
pub mod patch;
pub mod probability_rate;
pub mod utils;

use axum::{routing::get, Router};

use self::{
    banner::gacha_banner_list, patch::list_future_patch_date, probability_rate::handle,
};

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
}
