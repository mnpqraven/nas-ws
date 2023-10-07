use libsql_client::Client;
use nas_ws::handler::error::WorkerError;
use nas_ws::runnables::db::{seed_common, seed_table};

async fn clean_up(client: &Client) -> Result<(), WorkerError> {
    client
        .batch([
            "DROP TABLE IF EXISTS avatar",
            "DROP TABLE IF EXISTS item",
            "DROP TABLE IF EXISTS element",
            "DROP TABLE IF EXISTS path",
            "DROP TABLE IF EXISTS itemType",
            "DROP TABLE IF EXISTS itemSubType",
            "DROP TABLE IF EXISTS itemRarity",
        ])
        .await?;
    Ok(())
}

// TODO: clap
#[tokio::main]
pub async fn main() -> Result<(), WorkerError> {
    // clean_up(&client).await?;
    seed_common().await?;
    seed_table().await?;
    Ok(())
}
