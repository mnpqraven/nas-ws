use crate::routes::honkai::{
    dm_api::character::upstream_avatar_config::AvatarConfig, traits::DbData,
};
use std::collections::HashMap;

#[tokio::test]
async fn reading() {
    let t: HashMap<String, AvatarConfig> = AvatarConfig::read().await.unwrap();
    dbg!(t);
}
