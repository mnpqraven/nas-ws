use self::types::{Patch, PatchBanner};
use crate::handler::error::WorkerError;
use axum::Json;
use semver::Version;

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
    let banner_info: Vec<(Option<&str>, Option<&str>, Version)> = vec![
        (
            Some("Blade"),
            Some("Kafka"),
            Version::parse("1.2.0").unwrap(),
        ),
        (Some("Fu Xuan"), None, Version::parse("1.3.0").unwrap()),
    ];

    let patches = Patch::generate(5, None);
    let future_banners = PatchBanner::from_patches(patches, banner_info).await?;
    Ok(Json(future_banners))
}
