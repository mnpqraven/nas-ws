pub mod banner;
pub mod dm_api;
pub mod jade_estimate;
pub mod mhy_api;
pub mod patch;
pub mod probability_rate;
pub mod traits;
pub mod utils;

use self::dm_api::equipment_config::stat_ranking::stat_ranking;
use self::dm_api::equipment_config::{
    light_cone, light_cone_many, light_cone_promotion, light_cone_promotion_many, light_cone_skill,
    light_cone_skill_many,
};
use self::dm_api::skill_tree_config::trace;
use self::dm_api::{atlas, avatar_config, avatar_skill_config};
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
        .route("/mhy/eidolon/:char_id", get(internal::eidolon_by_char_id))
        .route("/mhy/attribute_property_list", get(properties))
        .route(
            "/light_cone/metadata",
            get(light_cone_many).post(light_cone_many),
        )
        .route("/light_cone/:id/metadata", get(light_cone))
        .route(
            "/light_cone/skill",
            get(light_cone_skill_many).post(light_cone_skill_many),
        )
        .route("/light_cone/:id/skill", get(light_cone_skill))
        .route(
            "/light_cone/promotion",
            get(light_cone_promotion_many).post(light_cone_promotion_many),
        )
        .route("/light_cone/:id/promotion", get(light_cone_promotion))
        .route("/light_cone/ranking", get(stat_ranking))
        .route("/signature_atlas", get(atlas::atlas_list))
        .route(
            "/avatar",
            get(avatar_config::character_many).post(avatar_config::character_many),
        )
        .route("/avatar/:id", get(avatar_config::character))
        .route("/avatar/:id/skill", get(avatar_skill_config::skill))
        .route("/skills", post(avatar_skill_config::skills))
        .route("/trace/:char_id", get(trace))
}
