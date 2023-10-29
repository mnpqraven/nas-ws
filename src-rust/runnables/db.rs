use tracing::info;

use crate::{
    builder::traits::DbAction,
    handler::error::WorkerError,
    routes::honkai::dm_api::{
        character::{eidolon::*, types::*},
        character_skill::types::*,
        equipment::{equipment_config::*, equipment_skill_config::*},
        item::types::*,
        types::*,
    },
};

pub async fn seed_common() -> Result<(), WorkerError> {
    info!("seeding common tables...");
    Path::seed().await?;
    Element::seed().await?;
    SkillType::seed().await?;

    ItemType::seed().await?;
    ItemSubType::seed().await?;
    ItemRarity::seed().await?;

    info!("common tables seeded!");
    Ok(())
}

pub async fn seed_table() -> Result<(), WorkerError> {
    info!("seeding main tables...");

    Item::seed().await?;

    AvatarConfig::seed().await?;
    AvatarSkillConfig::seed().await?;
    AvatarSkillTreeConfig::seed().await?;
    AvatarRankConfig::seed().await?;

    EquipmentSkillConfig::seed().await?;
    EquipmentConfig::seed().await?;

    info!("main tables seeded!");
    Ok(())
}
