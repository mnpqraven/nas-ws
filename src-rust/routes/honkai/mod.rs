pub mod banner;
pub mod dm_api;
pub mod jade_estimate;
pub mod patch;
pub mod probability_rate;
pub mod traits;
pub mod utils;

use self::dm_api::character::{character_by_name, character_many, eidolon, promotion};
use self::dm_api::equipment::stat_ranking::stat_ranking;
use self::dm_api::equipment::{
    lc_promotion, lc_promotions, lc_skill, lc_skills, light_cone, light_cone_search, light_cones,
};
use self::dm_api::equipment_skill::trace;
use self::dm_api::item::item_list;
use self::dm_api::property::property;
use self::dm_api::relic::{
    mainstat_spread, relic_set, relic_set_many, relic_set_search, relic_slot_type, relics_by_set,
    relics_by_set_post, set_bonus, set_bonus_many, substat_spread,
};
use self::dm_api::{atlas, character, character_skill};
use axum::routing::{get, post};
use axum::Router;

pub fn honkai_routes() -> Router {
    Router::new()
        .route("/jade_estimate", post(jade_estimate::handle))
        .route("/probability_rate", post(probability_rate::handle))
        .route("/patch_dates", get(banner::patch_date_list))
        .route("/patch_banners", get(banner::patch_banner_list))
        .route("/warp_banners", get(banner::warp_banner_list))
        .route("/properties", get(property))
        .route("/light_cone/search/:name", get(light_cone_search))
        .route("/light_cone/metadata", get(light_cones).post(light_cones))
        .route("/light_cone/:id/metadata", get(light_cone))
        .route("/light_cone/skill", get(lc_skills).post(lc_skills))
        .route("/light_cone/:id/skill", get(lc_skill))
        .route(
            "/light_cone/promotion",
            get(lc_promotions).post(lc_promotions),
        )
        .route("/light_cone/:id/promotion", get(lc_promotion))
        .route("/light_cone/ranking", get(stat_ranking))
        .route("/signature_atlas", get(atlas::atlas_list))
        .route("/avatar", get(character_many).post(character_many))
        .route("/avatar/:id", get(character::character))
        .route("/avatar/:id/skill", get(character_skill::skill))
        .route("/avatar/:id/trace_tree", get(character_skill::trace_tree))
        .route("/avatar/:id/trace", get(trace))
        .route("/avatar/:id/promotion", get(promotion))
        .route("/avatar/:id/eidolon", get(eidolon))
        .route("/character/search/:name", get(character_by_name))
        .route("/skills", post(character_skill::skills))
        .route("/relics", post(relics_by_set_post))
        .route("/relics/:setid", get(relics_by_set))
        .route("/relics/slot_type", post(relic_slot_type))
        .route("/relics/statspread/sub", get(substat_spread))
        .route("/relics/statspread/main", get(mainstat_spread))
        .route("/relic_set/bonus", get(set_bonus_many).post(set_bonus_many))
        .route("/relic_set/bonus/:id", get(set_bonus))
        .route("/relic_set", get(relic_set_many))
        .route("/relic_set/:id", get(relic_set))
        .route("/relic_set/search/:name", get(relic_set_search))
        .route("/item", get(item_list).post(item_list))
}
