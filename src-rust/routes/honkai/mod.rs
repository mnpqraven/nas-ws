pub mod constants;
pub mod gacha;
pub mod types;

use axum::{
    routing::{get, post},
    Router,
};

use self::gacha::{gacha_cfg, probability_rate};

pub fn honkai_routes() -> Router {
    Router::new().route("/gacha_cfg", get(gacha_cfg)).route(
        "/probability_rate",
        get(probability_rate).post(probability_rate),
    )
}
