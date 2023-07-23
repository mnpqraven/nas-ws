use self::types::AvatarSkillConfig;
use crate::{
    handler::error::WorkerError,
    routes::{endpoint_types::List, honkai::traits::DbData},
};
use axum::{extract::Path, Json};
use std::collections::HashMap;

use super::character::upstream_avatar_config::AvatarConfig;

pub mod types;

pub async fn skill(
    Path(character_id): Path<u32>,
) -> Result<Json<List<AvatarSkillConfig>>, WorkerError> {
    let skill_db: HashMap<u32, AvatarSkillConfig> = AvatarSkillConfig::read().await?;
    let character_db = AvatarConfig::read().await?;
    let character = character_db
        .get(&character_id)
        .ok_or(WorkerError::NotFound(character_id.to_string()))?;
    let skills = character.skill_list.clone();
    dbg!(&skills);

    let res: Vec<AvatarSkillConfig> = skills
        .iter()
        .map(|skill_id| {
            skill_db
                .get(skill_id)
                .ok_or(WorkerError::NotFound(skill_id.to_string()))
                .unwrap()
        })
        .cloned()
        .collect();

    Ok(Json(List::new(res)))
}

pub async fn skills(
    Json(skill_ids): Json<List<u32>>,
) -> Result<Json<List<AvatarSkillConfig>>, WorkerError> {
    let skill_db: HashMap<u32, AvatarSkillConfig> = AvatarSkillConfig::read().await?;
    let res: Vec<AvatarSkillConfig> = skill_ids
        .list
        .iter()
        .map(|key| {
            skill_db
                .get(key)
                .ok_or(WorkerError::NotFound(key.to_string()))
                .unwrap()
        })
        .cloned()
        .collect();

    Ok(Json(List::new(res)))
}
