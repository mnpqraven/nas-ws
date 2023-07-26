use crate::handler::{error::WorkerError, FromAxumResponse};
use axum::Json;
use fake::Dummy;
use fake::{Fake, Faker};
use response_derive::JsonResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use vercel_runtime::{Body, Response, StatusCode};

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema, Dummy)]
pub struct MvpAnalysis {
    data: HashMap<String, Vec<CharacterDamage>>,
}

#[derive(Debug, Deserialize, Serialize, JsonResponse, Clone, JsonSchema, Dummy)]
pub struct CharacterDamage {
    pub turn: u32,
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
    let mut map: HashMap<String, Vec<CharacterDamage>> = HashMap::new();
    let (mut qq, mut sw, mut nat, mut bronya) = (vec![], vec![], vec![], vec![]);
    for ind in 0..50 {
        let pnat = CharacterDamage {
            turn: ind,
            ..Faker.fake()
        };
        let psw = CharacterDamage {
            turn: ind,
            ..Faker.fake()
        };
        let pbronya = CharacterDamage {
            turn: ind,
            ..Faker.fake()
        };
        let pqq = CharacterDamage {
            turn: ind,
            team_distribution: InTeamDistribution {
                rate: 1.0
                    - pnat.team_distribution.rate
                    - psw.team_distribution.rate
                    - pbronya.team_distribution.rate,
            },
            ..Faker.fake()
        };

        qq.push(pqq);
        sw.push(psw);
        bronya.push(pbronya);
        nat.push(pnat);
    }
    map.insert("Quinque".to_owned(), qq);
    map.insert("Silver Wolf".to_owned(), sw);
    map.insert("Natasha".to_owned(), nat);
    map.insert("Bronya".to_owned(), bronya);

    Ok(Json(MvpAnalysis { data: map }))
}
