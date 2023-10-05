use libsql_client::{args, Config, Statement};
use nas_ws::builder::config::EnvConfig;
use nas_ws::handler::error::WorkerError;
use nas_ws::routes::honkai::dm_api::character::upstream_avatar_config::AvatarConfig;
use nas_ws::routes::honkai::mhy_api::types_parsed::shared::{Element, Path};
use nas_ws::routes::honkai::traits::DbData;
use strum::IntoEnumIterator;
use tracing::info;

#[allow(unused_variables, dead_code)]
pub async fn execute() -> Result<(), WorkerError> {
    let client = generate_db_client().await?;
    let rs = client.execute("select * from frameworks").await?;

    dbg!(rs);
    Ok(())
}

pub async fn generate_tables() -> Result<(), WorkerError> {
    let client = generate_db_client().await?;
    // element table
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS element (
                name TEXT PRIMARY KEY,
                type INTEGER NOT NULL
            )",
        )
        .await?;

    client
        .execute(
            "CREATE TABLE IF NOT EXISTS path (
                name TEXT PRIMARY KEY,
                type INTEGER NOT NULL
            )",
        )
        .await?;

    // avatar
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS avatar (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                rarity INTEGER NOT NULL,
                votag TEXT,
                damage_type TEXT NOT NULL,
                path TEXT NOT NULL,
                spneed INTEGER,
                FOREIGN KEY (damage_type) REFERENCES element(name),
                FOREIGN KEY (path) REFERENCES path(name)
            )",
        )
        .await?;

    let rs = client.execute("SELECT * FROM avatar").await?;
    dbg!(rs);

    Ok(())
}

async fn prepare_common() -> Result<(), WorkerError> {
    let client = generate_db_client().await?;

    // seed common tables
    let batch_element: Vec<Statement> = Element::iter()
        .map(|element| {
            Statement::with_args(
                "INSERT INTO element VALUES (?, ?)",
                args!(element.to_string(), element as i32),
            )
        })
        .collect();

    let batch_path: Vec<Statement> = Path::iter()
        .map(|path| {
            Statement::with_args(
                "INSERT INTO path VALUES (?, ?)",
                args!(path.to_string(), path as i32),
            )
        })
        .collect();

    let batches: Vec<Statement> = vec![batch_element, batch_path]
        .into_iter()
        .flatten()
        .collect();

    let _batch = client.batch(batches).await?;
    Ok(())
}

pub async fn seed() -> Result<(), WorkerError> {
    info!("seeding database...");
    let client = generate_db_client().await?;
    // seed avatars
    let avatar_db = AvatarConfig::read().await?;
    let batch_avatar: Vec<Statement> = avatar_db
        .into_values()
        .map(
            |AvatarConfig {
                 avatar_id,
                 avatar_name,
                 avatar_votag,
                 rarity,
                 damage_type,
                 avatar_base_type,
                 spneed,
                 ..
             }| {
                Statement::with_args(
                    "INSERT INTO avatar VALUES (?, ?, ?, ?, ?, ?, ?)",
                    args!(
                        avatar_id,
                        avatar_name,
                        rarity,
                        avatar_votag,
                        damage_type.to_string(),
                        avatar_base_type.to_string(),
                        spneed
                    ),
                )
            },
        )
        .collect();

    let _batch = client.batch(batch_avatar).await?;

    let rs = client.execute("SELECT * FROM avatar").await?;

    dbg!(rs);
    Ok(())
}

pub async fn generate_db_client() -> Result<libsql_client::client::Client, WorkerError> {
    let EnvConfig {
        db_url,
        db_auth_token,
    } = EnvConfig::new();

    let client = libsql_client::Client::from_config(Config {
        url: url::Url::parse(&db_url)?,
        auth_token: Some(db_auth_token),
    })
    .await?;

    Ok(client)
}

#[tokio::main]
pub async fn main() -> Result<(), WorkerError> {
    // generate_tables().await?;
    prepare_common().await?;
    seed().await?;
    Ok(())
}
