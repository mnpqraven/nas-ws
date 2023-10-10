use crate::{
    handler::{error::WorkerError, FromAxumResponse},
    routes::{
        endpoint_types::List,
        honkai::{
            dm_api::{
                atlas::{
                    avatar_atlas::UpstreamAvatarAtlas, equipment_atlas::UpstreamEquipmentAtlas,
                },
                character::{
                    promotion_config::AvatarPromotionConfig, types::AvatarConfig, eidolon::AvatarRankConfig,
                },
                character_skill::types::AvatarSkillConfig,
                equipment::{
                    equipment_config::*, equipment_promotion_config::*, equipment_skill_config::*,
                },
                equipment_skill::skill_tree_config::SkillTreeConfig,
                types::TextMap, property::config::AvatarPropertyConfig,
            },
            traits::DbData,
        },
    },
};
use axum::Json;
use response_derive::JsonResponse;
use serde::Serialize;
use tracing::info;
use vercel_runtime::{Body, Response, StatusCode};

#[derive(Serialize, JsonResponse, Debug)]
pub struct CronResult {
    pub task_name: String,
    pub success: bool,
}

impl CronResult {
    fn new(task_name: &str, success: bool) -> Self {
        Self {
            task_name: task_name.to_owned(),
            success,
        }
    }
}

pub async fn execute() -> Result<Json<List<CronResult>>, WorkerError> {
    info!("write_db ...");
    let avatar_db = AvatarConfig::try_write_disk().await.is_ok();
    let avatar_skill_db = AvatarSkillConfig::try_write_disk().await.is_ok();
    let eq_metadata_db = EquipmentConfig::try_write_disk().await.is_ok();
    let eq_skill_db = EquipmentSkillConfig::try_write_disk().await.is_ok();
    let eq_promotion_db = EquipmentPromotionConfig::try_write_disk().await.is_ok();

    let avatar_atlas = UpstreamAvatarAtlas::try_write_disk().await.is_ok();
    let equipment_atlas = UpstreamEquipmentAtlas::try_write_disk().await.is_ok();
    let eq_promotion = EquipmentPromotionConfig::try_write_disk().await.is_ok();
    let text_map = TextMap::try_write_disk().await.is_ok();
    let skill_tree_config = SkillTreeConfig::try_write_disk().await.is_ok();
    let db_character_eidolon = AvatarPromotionConfig::try_write_disk().await.is_ok();
    let property_config = AvatarPropertyConfig::try_write_disk().await.is_ok();
    let eidolon = AvatarRankConfig::try_write_disk().await.is_ok();

    Ok(Json(List::new(vec![
        CronResult::new("avatar_db", avatar_db),
        CronResult::new("avatar_skill_db", avatar_skill_db),
        CronResult::new("eq_metadata_db", eq_metadata_db),
        CronResult::new("eq_skill_db", eq_skill_db),
        CronResult::new("eq_promotion_db", eq_promotion_db),
        CronResult::new("avatar_atlas", avatar_atlas),
        CronResult::new("equipment_atlas", equipment_atlas),
        CronResult::new("eq_promotion", eq_promotion),
        CronResult::new("text_map", text_map),
        CronResult::new("skill_tree_config", skill_tree_config),
        CronResult::new("db_character_eidolon", db_character_eidolon),
        CronResult::new("property_config", property_config),
        CronResult::new("eidolon", eidolon),
    ])))
}
