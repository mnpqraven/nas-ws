use super::avatar_atlas::UpstreamAvatarAtlas;
use crate::routes::honkai::{dm_api::atlas::atlas_list, traits::DbData};
use axum::Json;

#[tokio::test]
async fn serde() {
    let t = UpstreamAvatarAtlas::read().await;
    assert!(t.is_ok());
}

#[tokio::test]
async fn ret() {
    let Json(t) = atlas_list().await.unwrap();
    dbg!(t);
}
