use self::types::{Banner, BannerList};
use crate::handler::error::WorkerError;
use axum::Json;

pub mod types;

pub async fn gacha_banner_list() -> Result<Json<BannerList>, WorkerError> {
    let banner_list = BannerList {
        banners: vec![
            Banner::char_ssr(),
            Banner::basic_weapon(),
            Banner::char_sr(),
            // dev_weapon uses unreleased pity systems
        ],
    };
    Ok(Json(banner_list))
}
