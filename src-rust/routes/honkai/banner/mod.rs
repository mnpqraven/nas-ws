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

pub async fn patch_banner_list() -> Result<Json<List<PatchBanner>>, WorkerError> {
    let now = std::time::Instant::now();

    let banner_info: Vec<(Option<&str>, Option<&str>, Version)> = BANNER_CHARS
        .into_iter()
        .map(|(version, char1, char2)| {
            let version = Version::parse(version).unwrap();
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
        ("Even Immortality Ends", Version::parse("1.2.0")?),
        (
            "Celestial Eyes Above Mortal Ruins",
            Version::parse("1.3.0")?,
        ),
        ("Jolted Awake From a Winter Dream", Version::parse("1.4.0")?),
    ];

    let future_patches = Patch::generate(5, Some(patches_info));
    Ok(Json(future_patches.into()))
}
