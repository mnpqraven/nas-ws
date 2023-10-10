use nas_ws::builder::get_db_client;
use nas_ws::handler::error::WorkerError;
use nas_ws::runnables::db::{seed_common, seed_table};

// TODO: teardown
async fn teardown() -> Result<(), WorkerError> {
    let client = get_db_client().await?;
    client
        .batch([
            // "DROP TABLE IF EXISTS avatar",
            // "DROP TABLE IF EXISTS item",
            // "DROP TABLE IF EXISTS element",
            // "DROP TABLE IF EXISTS path",
            // "DROP TABLE IF EXISTS itemType",
            // "DROP TABLE IF EXISTS itemSubType",
            // "DROP TABLE IF EXISTS itemRarity",
            "DROP TABLE IF EXISTS avatarSkill",
        ])
        .await?;
    Ok(())
}

// TODO: clap
#[tokio::main]
pub async fn main() -> Result<(), WorkerError> {
    // teardown().await?;
    seed_common().await?;
    seed_table().await?;
    Ok(())
}
