use crate::{
    handler::error::WorkerError, routes::honkai::dm_api::character_skill::types::AvatarSkillConfig,
};
use std::path::Path;
use tokio::process::Command;
use tracing::{error, info};

pub async fn execute() -> Result<(), WorkerError> {
    info!("spinning up DM repo...");
    if Path::new("../StarRailData").exists() {
        info!("local DM directory exists, attempting pull...");
        let _pull = Command::new("git")
            .args(["-C", "../StarRailData", "pull"])
            .output()
            .await?;
        info!("pull completed");
        return Ok(());
    }
    info!("local DM directory does not exist, attempting clone...");
    let _clone = Command::new("git")
        .args([
            "clone",
            "https://github.com/Dimbreath/StarRailData.git",
            "../StarRailData",
        ])
        .output()
        .await
        .map_err(|_| {
            error!("cloning failed");
            WorkerError::ServerSide
        });
    info!("DM repo cloned");
    Ok(())
}

pub async fn chunk_splitter() -> Result<(), WorkerError> {
    info!("attempting file splitting...");
    AvatarSkillConfig::write_splitted().await?;
    info!("file splitting completed");
    Ok(())
}
