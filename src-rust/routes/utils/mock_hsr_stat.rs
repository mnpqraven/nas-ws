use crate::handler::{error::WorkerError, FromAxumResponse};
use axum::Json;
use fake::Dummy;
use fake::{Fake, Faker};
use response_derive::JsonResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vercel_runtime::{Body, Response, StatusCode};

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema, Dummy)]
pub struct MvpAnalysis {
    data: Vec<[CharacterDamage; 4]>,
}

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema, Dummy)]
pub struct CharacterDamage {
    pub name: String,
    pub team_distribution: InTeamDistribution,
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
    #[dummy(faker = "0.0 .. 0.25")]
    pub rate: f32,
}

pub(super) async fn handle() -> Result<Json<MvpAnalysis>, WorkerError> {
    // let names = ["Qingque", "Silver Wolf", "Natasha", "Bronya"];
    let mut data: Vec<[CharacterDamage; 4]> = vec![];
    for _ in 0..50 {
        let value = generate_mock();
        data.push(value);
    }

    Ok(Json(MvpAnalysis { data }))
}

fn generate_mock() -> [CharacterDamage; 4] {
    let qingque = CharacterDamage {
        name: "Qingque".to_string(),
        ..Faker.fake()
    };
    let sw = CharacterDamage {
        name: "Silver Wolf".to_string(),
        ..Faker.fake()
    };
    let nat = CharacterDamage {
        name: "Natasha".to_string(),
        ..Faker.fake()
    };
    let rest_rate = 1.0
        - qingque.team_distribution.rate
        - sw.team_distribution.rate
        - nat.team_distribution.rate;

    let bronya = CharacterDamage {
        name: "Bronya".to_string(),
        team_distribution: InTeamDistribution { rate: rest_rate },
        ..Faker.fake()
    };
    [qingque, sw, nat, bronya]
}
