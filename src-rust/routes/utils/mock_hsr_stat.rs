use axum::Json;
use fake::{Fake, Faker};
use tracing::instrument;

use crate::{
    handler::error::WorkerError, routes::honkai::mhy_api::types_parsed::character::Character,
};

#[instrument(ret, err)]
pub async fn handle() -> Result<Json<Character>, WorkerError> {
    let randomized_character: Character = Faker.fake();

    Ok(Json(randomized_character))
}
