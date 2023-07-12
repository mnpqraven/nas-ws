use crate::routes::honkai::{
    dm_api::avatar_config::upstream_avatar_config::UpstreamAvatarConfig,
    mhy_api::internal::impls::DbData,
};

#[tokio::test]
async fn reading() {
    let t = <UpstreamAvatarConfig as DbData<UpstreamAvatarConfig>>::read()
        .await
        .unwrap();
    dbg!(t);
}
