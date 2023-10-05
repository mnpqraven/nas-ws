use libsql_client::{args, Client, Config, Statement};
use nas_ws::builder::config::EnvConfig;
use nas_ws::handler::error::WorkerError;
use nas_ws::routes::honkai::dm_api::character::upstream_avatar_config::AvatarConfig;
use nas_ws::routes::honkai::dm_api::item::types::{Item, ItemType, ItemSubType, ItemRarity};
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

pub async fn generate_tables(client: &Client) -> Result<(), WorkerError> {
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

async fn prepare_common(client: &Client) -> Result<(), WorkerError> {
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

pub async fn seed_avatar(client: &Client) -> Result<(), WorkerError> {
    info!("seeding database...");
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

    client.batch(batch_avatar).await?;

    Ok(())
}

pub async fn seed_item(client: &Client) -> Result<(), WorkerError> {
    let item_db = Item::read().await?;
    let batch_type: Vec<Statement> = ItemType::iter()
        .enumerate()
        .map(|(i, value)| {
            Statement::with_args(
                "INSERT INTO itemType VALUES (?, ?)",
                args!(value.to_string(), i),
            )
        })
        .collect();
    let batch_sub_type: Vec<Statement> = ItemSubType::iter()
        .enumerate()
        .map(|(i, value)| {
            Statement::with_args(
                "INSERT INTO itemSubType VALUES (?, ?)",
                args!(value.to_string(), i),
            )
        })
        .collect();
    let batch_rarity: Vec<Statement> = ItemRarity::iter()
        .enumerate()
        .map(|(i, value)| {
            Statement::with_args(
                "INSERT INTO itemRarity VALUES (?, ?)",
                args!(value.to_string(), i),
            )
        })
        .collect();

    let batch_item: Vec<Statement> = item_db
        .into_values()
        .map(|item| {
            Statement::with_args(
                "INSERT INTO item VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                args!(
                    item.id,
                    item.item_name,
                    item.rarity.to_string(),
                    item.item_main_type.to_string(),
                    item.item_sub_type.to_string(),
                    item.inventory_display_tag,
                    item.purpose_type,
                    item.item_desc,
                    item.item_bgdesc,
                    item.pile_limit
                ),
            )
        })
        .collect();

    let batch: Vec<Statement> = [batch_type, batch_sub_type, batch_rarity, batch_item]
        .into_iter()
        .flatten()
        .collect();

    client.batch(batch).await?;
    Ok(())
}

pub async fn generate_db_client() -> Result<Client, WorkerError> {
    let env = EnvConfig::new();

    let client = Client::from_config(Config {
        url: url::Url::parse(&env.db_url)?,
        auth_token: Some(env.db_auth_token),
    })
    .await?;

    Ok(client)
}

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

#[tokio::main]
pub async fn main() -> Result<(), WorkerError> {
    let client = generate_db_client().await?;

    // clean_up(&client).await?;
    // generate_tables(&client).await?;
    prepare_common(&client).await?;
    seed_avatar(&client).await?;
    seed_item(&client).await?;
    Ok(())
}
