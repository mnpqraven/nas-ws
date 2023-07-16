use crate::{
    handler::{error::WorkerError, FromAxumResponse},
    routes::honkai::{
        dm_api::{
            avatar_config::upstream_avatar_config::AvatarConfig,
            avatar_skill_config::types::AvatarSkillConfig,
            equipment_config::{
                equipment_config::*, equipment_promotion_config::*, equipment_skill_config::*,
            },
        },
        mhy_api::internal::categorizing::*,
        traits::DbData,
    },
};
use axum::Json;
use response_derive::JsonResponse;
use serde::Serialize;
use tracing::info;
use vercel_runtime::{Body, Response, StatusCode};

#[derive(Serialize, JsonResponse, Debug)]
pub struct CronResult {
    pub avatar_db: bool,
    pub skill_db: bool,
    pub trace_db: bool,
    pub lc_db: bool,
    pub avatar_skill_db: bool,
    pub eq_metadata_db: bool,
    pub eq_skill_db: bool,
    pub eq_promotion_db: bool,
}

pub async fn write_db() -> Result<Json<CronResult>, WorkerError> {
    info!("write_db ...");
    let skill_db = <DbCharacterSkill as DbData<DbCharacterSkill>>::try_write_disk().await;
    let trace_db = <DbCharacterSkillTree as DbData<DbCharacterSkillTree>>::try_write_disk().await;

    let lc_db = <EquipmentConfig as DbData<EquipmentConfig>>::try_write_disk().await;
    let avatar_db = <AvatarConfig as DbData<AvatarConfig>>::try_write_disk().await;
    let avatar_skill_db = <AvatarSkillConfig as DbData<AvatarSkillConfig>>::try_write_disk().await;
    let eq_metadata_db = <EquipmentConfig as DbData<EquipmentConfig>>::try_write_disk().await;
    let eq_skill_db =
        <EquipmentSkillConfig as DbData<EquipmentSkillConfig>>::try_write_disk().await;
    let eq_promotion_db =
        <EquipmentPromotionConfig as DbData<EquipmentPromotionConfig>>::try_write_disk().await;

    Ok(Json(CronResult {
        skill_db: skill_db.is_ok(),
        trace_db: trace_db.is_ok(),
        avatar_db: avatar_db.is_ok(),
        lc_db: lc_db.is_ok(),
        avatar_skill_db: avatar_skill_db.is_ok(),
        eq_metadata_db: eq_metadata_db.is_ok(),
        eq_skill_db: eq_skill_db.is_ok(),
        eq_promotion_db: eq_promotion_db.is_ok(),
    }))
}
