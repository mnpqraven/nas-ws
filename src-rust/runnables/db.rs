use tracing::info;

use crate::{
    builder::traits::DbAction,
    handler::error::WorkerError,
    routes::honkai::{
        dm_api::{character::upstream_avatar_config::*, character_skill::types::*, item::types::*},
        mhy_api::{internal::categorizing::SkillType, types_parsed::shared::*},
    },
};

pub async fn seed_common() -> Result<(), WorkerError> {
    info!("seeding common tables...");
    // Path::seed().await?;
    // Element::seed().await?;
    // SkillType::seed().await?;

    // ItemType::seed().await?;
    // ItemSubType::seed().await?;
    // ItemRarity::seed().await?;

    info!("common tables seeded!");
    Ok(())
}

pub async fn seed_table() -> Result<(), WorkerError> {
    info!("seeding main tables...");
    // AvatarConfig::seed().await?;
    // Item::seed().await?;
    // AvatarSkillConfig::seed().await?;
    AvatarSkillTreeConfig::seed().await?;

    info!("main tables seeded!");
    Ok(())
}
