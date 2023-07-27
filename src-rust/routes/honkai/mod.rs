pub mod banner;
pub mod dm_api;
pub mod jade_estimate;
pub mod mhy_api;
pub mod patch;
pub mod probability_rate;
pub mod traits;
pub mod utils;

use self::dm_api::character::{character_by_name, character_many, eidolon, promotion};
use self::dm_api::equipment::stat_ranking::stat_ranking;
use self::dm_api::equipment::{
    light_cone, light_cone_many, light_cone_promotion, light_cone_promotion_many,
    light_cone_search, light_cone_skill, light_cone_skill_many,
};
use self::dm_api::equipment_skill::trace;
use self::dm_api::property::property;
use self::dm_api::relic::{relic_set, relic_set_search, relics_by_set, set_bonus, set_bonus_many, relic_set_many};
use self::dm_api::{atlas, character, character_skill};
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
        // .route("/mhy/character", get(internal::all_characters))
        // .route("/mhy/character/:id", get(internal::character_by_id))
        // .route("/mhy/eidolon/:char_id", get(internal::eidolon_by_char_id))
        // .route("/mhy/attribute_property_list", get(properties))
        .route("/properties", get(property))
        .route("/light_cone/search/:name", get(light_cone_search))
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
        .route("/avatar", get(character_many).post(character_many))
        .route("/avatar/:id", get(character::character))
        .route("/avatar/:id/skill", get(character_skill::skill))
        .route("/avatar/:id/trace", get(trace))
        .route("/avatar/:id/promotion", get(promotion))
        .route("/avatar/:id/eidolon", get(eidolon))
        .route("/character/search/:name", get(character_by_name))
        .route("/skills", post(character_skill::skills))
        .route("/relics/:setid", get(relics_by_set))
        .route("/relic_set/bonus", get(set_bonus_many).post(set_bonus_many))
        .route("/relic_set/bonus/:id", get(set_bonus))
        .route("/relic_set", get(relic_set_many))
        .route("/relic_set/:id", get(relic_set))
        .route("/relic_set/search/:name", get(relic_set_search))
}
