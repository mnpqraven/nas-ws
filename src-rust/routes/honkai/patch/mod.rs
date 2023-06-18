use self::types::{Patch, PatchBanner};
use crate::handler::error::WorkerError;
use axum::Json;
use semver::Version;
use tracing::info;

#[cfg(test)]
mod tests;
pub mod types;
pub mod future_date;
pub mod future_banner;

pub async fn list_future_patch_date() -> Result<Json<Vec<Patch>>, WorkerError> {
    let patches_info: Vec<(&str, Version)> = vec![
        // ("Dank", Version::parse("1.2.0").unwrap()),
        // ("Meme", Version::parse("1.3.0").unwrap()),
    ];

    let future_patches = Patch::generate(5, Some(patches_info));
    tracing::info!("{:?}", future_patches);
    Ok(Json(future_patches))
}

pub async fn list_future_patch_banner() -> Result<Json<Vec<PatchBanner>>, WorkerError> {
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
    Ok(Json(future_banners))
}
