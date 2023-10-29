use crate::routes::honkai::{
    dm_api::character::types::AvatarConfig, traits::DbData,
};
use std::collections::HashMap;

#[tokio::test]
async fn reading() {
    let t: HashMap<u32, AvatarConfig> = AvatarConfig::read().await.unwrap();
    dbg!(t);
}
