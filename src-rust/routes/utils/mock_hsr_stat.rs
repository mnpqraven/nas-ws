use crate::handler::{error::WorkerError, FromAxumResponse};
use crate::routes::honkai::mhy_api::types_parsed::character::Character;
use axum::Json;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Dummy;
use fake::{Fake, Faker};
use response_derive::JsonResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use vercel_runtime::{Body, Response, StatusCode};

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema, Dummy)]
pub struct MvpWrapper {
    pub character: Character,
    pub team_distribution: Vec<[InTeamDistribution; 4]>,
    pub self_distribution: DamageSelfDistribution,
}

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema, Dummy)]
pub struct DamageSelfDistribution {
    // % dist, count, avg.min, avg.max
    #[dummy(faker = "((0.0..1.0), (1..100), (1000..10000), (10000..20000))")]
    pub skill: (f32, u32, u32, u32),
    #[dummy(faker = "((0.0..1.0), (1..100), (30000..40000), (40000..80000))")]
    pub ult: (f32, u32, u32, u32),
    #[dummy(faker = "((0.0..1.0), (1..100), (1000..2000), (2000..3000))")]
    pub basic: (f32, u32, u32, u32),
    #[dummy(faker = "((0.0..1.0), (1..100), (1000..10000), (10000..20000))")]
    pub followup: (f32, u32, u32, u32),
}
#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema, Dummy)]
pub struct InTeamDistribution {
    #[dummy(faker = "Name(EN)")]
    pub name: String,
    #[dummy(faker = "0.0 .. 1.0")]
    pub rate: f32,
}

#[instrument(ret, err)]
pub(super) async fn handle() -> Result<Json<MvpWrapper>, WorkerError> {
    let randomized_character: MvpWrapper = Faker.fake();

    Ok(Json(randomized_character))
}
