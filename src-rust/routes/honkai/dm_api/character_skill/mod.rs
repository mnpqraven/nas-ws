use self::types::AvatarSkillConfig;
use crate::{
    handler::error::WorkerError,
    routes::{endpoint_types::List, honkai::traits::DbData},
};
use axum::{extract::Path, Json};
use tracing::info;

use super::character::upstream_avatar_config::AvatarConfig;

pub mod types;

pub async fn skill(
    Path(character_id): Path<u32>,
) -> Result<Json<List<AvatarSkillConfig>>, WorkerError> {
    let now = std::time::Instant::now();

    let character_db = AvatarConfig::read().await?;
    let character = character_db
        .get(&character_id)
        .ok_or(WorkerError::NotFound(character_id.to_string()))?;
    let skills = character.skill_list.clone();

    let res: Vec<AvatarSkillConfig> = skills
        .into_iter()
        .map(|skill_id| AvatarSkillConfig::read_splitted_by_skillid(skill_id).unwrap())
        .collect();

    info!("[/skill/:id] character_skill: {:?}", now.elapsed());
    Ok(Json(List::new(res)))
}

pub async fn skills(
    Json(skill_ids): Json<List<u32>>,
) -> Result<Json<List<AvatarSkillConfig>>, WorkerError> {
    let res: Vec<AvatarSkillConfig> = skill_ids
        .list
        .into_iter()
        .map(|key| AvatarSkillConfig::read_splitted_by_skillid(key).unwrap())
        .collect();

    Ok(Json(List::new(res)))
}
