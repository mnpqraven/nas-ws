pub mod gacha_cfg;
pub mod types;

use axum::{routing::get, Router};

pub fn honkai_routes() -> Router {
    Router::new().route("/gacha_cfg", get(gacha_cfg::gacha_cfg))
}
