use super::shared::AssetPath;
use crate::handler::{error::WorkerError, FromAxumResponse};
use axum::Json;
use response_derive::JsonResponse;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use vercel_runtime::{Body, Response, StatusCode};

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
pub struct Player {
    uid: String,
    nickname: String,
    level: u32,
    world_level: u32, //  EQ
    friend_count: u32,
    avatar: Avatar,
    signature: String,
    is_display: bool,
    space_info: SpaceInfo,
}

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
pub struct SpaceInfo {
    challenge_data: ChallengeData,
    pass_area_progress: u32,
    light_cone_count: u32,
    avatar_count: u32,
    achievement_count: u32,
}

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
pub struct ChallengeData {
    maze_group_id: u32,
    maze_group_index: u32,
    pre_maze_group_index: u32,
}

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema)]
pub struct Avatar {
    id: String,
    name: String,
    icon: AssetPath,
}
