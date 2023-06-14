use self::types::{BannerList, PatchList};
use crate::handler::error::WorkerError;
use axum::Json;
use semver::Version;

pub mod types;

pub async fn list_future_patch_date() -> Result<Json<PatchList>, WorkerError> {
    let patches_info: Vec<(&str, Version)> = vec![
        // ("Dank", Version::parse("1.2.0").unwrap()),
        // ("Meme", Version::parse("1.3.0").unwrap()),
    ];

    let future_patches = PatchList::generate(5, Some(patches_info));
    tracing::info!("{:?}", future_patches);
    Ok(Json(future_patches))
}

pub async fn list_future_patch_banner() -> Result<Json<BannerList>, WorkerError> {
    let banner_info: Vec<(Option<&str>, Option<&str>, Version)> = vec![
        (
            Some("Blade"),
            Some("Kafka"),
            Version::parse("1.2.0").unwrap(),
        ),
        (Some("Fu Xuan"), None, Version::parse("1.3.0").unwrap()),
    ];

    let future_patches = PatchList::generate(5, None);
    let future_banners = BannerList::from_patches(future_patches.patches, banner_info);
    Ok(Json(future_banners))
}
