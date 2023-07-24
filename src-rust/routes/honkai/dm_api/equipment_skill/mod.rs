use self::skill_tree_config::SkillTreeConfig;
use crate::{
    handler::error::WorkerError,
    routes::{endpoint_types::List, honkai::traits::DbData},
};
use axum::{extract::Path, Json};
use tracing::info;

pub mod skill_tree_config;

pub async fn trace(Path(char_id): Path<u32>) -> Result<Json<List<SkillTreeConfig>>, WorkerError> {
    let now = std::time::Instant::now();

    let trace_db = SkillTreeConfig::read().await?;
    info!("DB Read: {:.2?}", now.elapsed());

    let res: Vec<SkillTreeConfig> = trace_db
        .iter()
        .filter(|(_, v)| v.avatar_id == char_id)
        .map(|(_, v)| v.clone())
        .collect();

    info!("Total elapsed: {:.2?}", now.elapsed());
    Ok(Json(List::new(res)))
}
