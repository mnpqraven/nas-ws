use self::types::Banner;
use super::patch::types::PatchBanner;
use crate::{
    handler::error::WorkerError,
    routes::{
        endpoint_types::List,
        honkai::{banner::constants::BANNER_CHARS, patch::types::Patch},
    },
};
use axum::Json;
use semver::Version;
use tracing::{info, instrument};

mod constants;
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

    let mut first_version = Version::parse("1.1.0").unwrap();

    let banner_info: Vec<(Option<&str>, Option<&str>, Version)> = BANNER_CHARS
        .iter()
        .map(|(char1, char2)| {
            let version = first_version.clone();
            first_version.minor += 1;
            (char1, char2, version)
        })
        .collect();

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
