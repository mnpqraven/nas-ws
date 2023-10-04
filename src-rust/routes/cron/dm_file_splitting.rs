use crate::{
    handler::error::WorkerError,
    routes::honkai::dm_api::{
        character_skill::types::{AvatarSkillConfig, AvatarSkillTreeConfig},
        relic::config::RelicConfig,
    },
};
use tracing::info;

pub async fn execute() -> Result<(), WorkerError> {
    info!("attempting file splitting...");
    AvatarSkillConfig::write_splitted().await?;
    AvatarSkillTreeConfig::write_splitted().await?;
    RelicConfig::write_splitted().await?;
    info!("file splitting completed");
    Ok(())
}
