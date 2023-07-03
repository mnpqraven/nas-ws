pub mod banner;
pub mod dm_api;
pub mod jade_estimate;
pub mod mhy_api;
pub mod patch;
pub mod probability_rate;
pub mod utils;

use self::mhy_api::internal::{self, properties};
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
        .route("/patch_dates", get(banner::patch_date_list))
        .route("/patch_banners", get(banner::patch_banner_list))
        .route("/warp_banners", get(banner::warp_banner_list))
        .route("/mhy", post(mhy_api::handle))
        .route("/mhy/character", get(internal::all_characters))
        .route("/mhy/character/:id", get(internal::character_by_id))
        .route("/mhy/trace/:char_id", get(internal::trace_by_char_id))
        .route("/mhy/big_trace/:char_id", get(dm_api::read_by_char_id))
        .route("/mhy/eidolon/:char_id", get(internal::eidolon_by_char_id))
        .route("/mhy/skill/:id", get(internal::skill_by_char_id))
        .route("/mhy/attribute_property_list", get(properties))
        .route("/mhy/light_cone/:id", get(dm_api::light_cone_by_id)) // db data

}
