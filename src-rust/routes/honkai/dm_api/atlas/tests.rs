use super::avatar_atlas::UpstreamAvatarAtlas;
use crate::routes::honkai::dm_api::atlas::main_ret;
use crate::routes::honkai::mhy_api::internal::impls::DbData;

#[tokio::test]
async fn serde() {
    let t = <UpstreamAvatarAtlas as DbData<UpstreamAvatarAtlas>>::read().await;
    assert!(t.is_ok());
}

#[tokio::test]
async fn ret() {
    let t = main_ret().await.unwrap();
    dbg!(t);
}
