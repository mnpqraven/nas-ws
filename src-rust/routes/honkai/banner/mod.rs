use self::types::Banner;
use super::patch::types::PatchBanner;
use crate::{
    handler::error::WorkerError,
    routes::{endpoint_types::List, honkai::patch::types::Patch},
};
use axum::Json;
use semver::Version;
use tracing::{info, instrument};

pub mod types;

#[instrument(ret, err)]
pub async fn warp_banner_list() -> Result<Json<List<Banner>>, WorkerError> {
    let banner_list = List {
        list: vec![
            Banner::char_ssr(),
            Banner::basic_weapon(),
            Banner::char_sr(),
            // dev_weapon uses unreleased pity systems
        ],
    };
    Ok(Json(banner_list))
}

#[instrument(ret, err)]
pub async fn patch_banner_list() -> Result<Json<List<PatchBanner>>, WorkerError> {
    let now = std::time::Instant::now();
    let banner_info: Vec<(Option<&str>, Option<&str>, Version)> = vec![
        (
            Some("Silver Wolf"),
            Some("Luocha"),
            Version::parse("1.1.0").unwrap(),
        ),
        (
            Some("Blade"),
            Some("Kafka"),
            Version::parse("1.2.0").unwrap(),
        ),
        (Some("Fu Xuan"), None, Version::parse("1.3.0").unwrap()),
    ];

    let patches = Patch::generate(5, None);
    let future_banners = PatchBanner::from_patches(patches, banner_info).await?;
    info!("Total elapsed: {:.2?}", now.elapsed());
    Ok(Json(future_banners.into()))
}

#[instrument(ret, err)]
pub async fn patch_date_list() -> Result<Json<List<Patch>>, WorkerError> {
    let patches_info: Vec<(&str, Version)> = vec![
        // ("Dank", Version::parse("1.2.0").unwrap()),
        // ("Meme", Version::parse("1.3.0").unwrap()),
    ];

    let future_patches = Patch::generate(5, Some(patches_info));
    Ok(Json(future_patches.into()))
}
