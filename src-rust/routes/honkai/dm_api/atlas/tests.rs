use super::avatar_atlas::UpstreamAvatarAtlas;
use crate::routes::honkai::{dm_api::atlas::atlas_list, mhy_api::internal::impls::DbData};
use axum::Json;

#[tokio::test]
async fn serde() {
    let t = <UpstreamAvatarAtlas as DbData<UpstreamAvatarAtlas>>::read().await;
    assert!(t.is_ok());
}

#[tokio::test]
async fn ret() {
    let Json(t) = atlas_list().await.unwrap();
    dbg!(t);
}
