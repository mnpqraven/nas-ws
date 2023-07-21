use crate::{
    handler::{error::WorkerError, FromAxumResponse},
    routes::{
        endpoint_types::List,
        honkai::{
            dm_api::{
                atlas::{
                    avatar_atlas::UpstreamAvatarAtlas, equipment_atlas::UpstreamEquipmentAtlas,
                },
                avatar_config::upstream_avatar_config::AvatarConfig,
                avatar_skill_config::types::AvatarSkillConfig,
                equipment_config::{
                    equipment_config::*, equipment_promotion_config::*, equipment_skill_config::*,
                },
                skill_tree_config::skill_tree_config::SkillTreeConfig,
                types::TextMap,
            },
            mhy_api::{internal::categorizing::*, types_parsed::shared::DbAttributeProperty},
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

pub async fn write_db() -> Result<Json<List<CronResult>>, WorkerError> {
    info!("write_db ...");
    let skill_db = <DbCharacterSkill as DbData<DbCharacterSkill>>::try_write_disk().await.is_ok();
    let trace_db = <DbCharacterSkillTree as DbData<DbCharacterSkillTree>>::try_write_disk().await.is_ok();

    let avatar_db = <AvatarConfig as DbData<AvatarConfig>>::try_write_disk().await.is_ok();
    let avatar_skill_db = <AvatarSkillConfig as DbData<AvatarSkillConfig>>::try_write_disk().await.is_ok();
    let eq_metadata_db = <EquipmentConfig as DbData<EquipmentConfig>>::try_write_disk().await.is_ok();
    let eq_skill_db = <EquipmentSkillConfig as DbData<EquipmentSkillConfig>>::try_write_disk().await.is_ok();
    let eq_promotion_db = <EquipmentPromotionConfig as DbData<EquipmentPromotionConfig>>::try_write_disk().await.is_ok();

    let avatar_atlas = <UpstreamAvatarAtlas as DbData<UpstreamAvatarAtlas>>::try_write_disk().await.is_ok();
    let equipment_atlas = <UpstreamEquipmentAtlas as DbData<UpstreamEquipmentAtlas>>::try_write_disk().await.is_ok();
    let eq_promotion = <EquipmentPromotionConfig as DbData<EquipmentPromotionConfig>>::try_write_disk().await.is_ok();
    let text_map = <TextMap as DbData<TextMap>>::try_write_disk().await.is_ok();
    let skill_tree_config = <SkillTreeConfig as DbData<SkillTreeConfig>>::try_write_disk().await.is_ok();
    let db_character_eidolon = <DbCharacterEidolon as DbData<DbCharacterEidolon>>::try_write_disk().await.is_ok();
    let attribute_property = <DbAttributeProperty as DbData<DbAttributeProperty>>::try_write_disk().await.is_ok();

    Ok(Json(List::new(vec![
        CronResult::new("skill_db", skill_db),
        CronResult::new("trace_db", trace_db),
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
        CronResult::new("attribute_property", attribute_property),
    ])))
}
