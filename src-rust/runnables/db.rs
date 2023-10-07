use tracing::info;

use crate::{
    builder::traits::DbAction,
    handler::error::WorkerError,
    routes::honkai::{
        dm_api::{character::upstream_avatar_config::AvatarConfig, item::types::*},
        mhy_api::types_parsed::shared::*,
    },
};

pub async fn seed_common() -> Result<(), WorkerError> {
    info!("seeding common tables...");
    Path::seed().await?;
    Element::seed().await?;

    ItemType::seed().await?;
    ItemSubType::seed().await?;
    ItemRarity::seed().await?;

    info!("common tables seeded!");
    Ok(())
}

pub async fn seed_table() -> Result<(), WorkerError> {
    AvatarConfig::seed().await?;
    Item::seed().await?;

    Ok(())
}
