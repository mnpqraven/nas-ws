use self::types::AvatarSkillConfig;
use crate::{
    handler::error::WorkerError,
    routes::{endpoint_types::List, honkai::traits::DbData},
};
use axum::{extract::Path, Json};
use std::collections::HashMap;

pub mod types;

pub async fn skill(
    Path(character_id): Path<u32>,
) -> Result<Json<List<AvatarSkillConfig>>, WorkerError> {
    let skill_db: HashMap<String, AvatarSkillConfig> = AvatarSkillConfig::read().await?;
    let res: Vec<AvatarSkillConfig> = skill_db
        .iter()
        .filter(|(_, v)| {
            v.skill_id
                .to_string()
                .starts_with(&character_id.to_string())
        })
        .map(|(_, v)| v.clone())
        .collect();

    Ok(Json(List::new(res)))
}

pub async fn skills(
    Json(skill_ids): Json<List<u32>>,
) -> Result<Json<List<AvatarSkillConfig>>, WorkerError> {
    let skill_db: HashMap<String, AvatarSkillConfig> = AvatarSkillConfig::read().await?;
    let skill_ids: Vec<String> = skill_ids.list.iter().map(|e| e.to_string()).collect();
    dbg!(&skill_ids);
    let res: Vec<AvatarSkillConfig> = skill_db
        .iter()
        .filter(|(key, _)| {
            dbg!(&key);
            skill_ids.contains(&key)
        })
        .map(|(_, v)| v.clone())
        .collect();

    Ok(Json(List::new(res)))
}
