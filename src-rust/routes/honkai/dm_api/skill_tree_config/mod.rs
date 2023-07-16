use std::collections::HashMap;

use self::skill_tree_config::SkillTreeConfig;
use crate::{
    handler::error::WorkerError,
    routes::{endpoint_types::List, honkai::traits::DbData},
};
use axum::{extract::Path, Json};

pub mod skill_tree_config;

pub async fn trace(Path(char_id): Path<u32>) -> Result<Json<List<SkillTreeConfig>>, WorkerError> {
    let trace_db: HashMap<String, SkillTreeConfig> = SkillTreeConfig::read().await?;

    let res: Vec<SkillTreeConfig> = trace_db
        .iter()
        .filter(|(_, v)| v.avatar_id == char_id)
        .map(|(_, v)| v.clone())
        .collect();

    Ok(Json(List::new(res)))
}
