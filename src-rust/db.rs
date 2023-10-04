use libsql_client::{args, Config, Statement};
use nas_ws::builder::config::EnvConfig;
use nas_ws::handler::error::WorkerError;
use tracing::info;

#[allow(unused_variables, dead_code)]
pub async fn execute() -> Result<(), WorkerError> {
    let client = generate_db_client().await?;

    // let a_binding =
    //     client.execute("CREATE TABLE IF NOT EXISTS example ( uid TEXT PRIMARY KEY, email TEXT )");
    // let a = a_binding.await?;
    // dbg!(a);
    // let b_binding = client.execute("INSERT INTO example VALUES ('uid1', 'foo@bar.com')");
    // let b = b_binding.await?;
    // dbg!(b);
    let rs = client.execute("select * from frameworks").await?;

    dbg!(rs);
    Ok(())
}

pub async fn generate_tables() -> Result<(), WorkerError> {
    let client = generate_db_client().await?;
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS avatars (
                id INTEGER PRIMARY KEY,
                name TEXT,
                votag TEXT,
                spneed INTEGER
            )",
        )
        .await?;

    let rs = client.execute("SELECT * FROM avatars").await?;
    dbg!(rs);

    Ok(())
}
pub async fn seed() -> Result<(), WorkerError> {
    info!("seeding database...");
    let client = generate_db_client().await?;
    client
        .execute(Statement::with_args(
            "INSERT INTO avatars VALUES (?, ?, ?, ?)",
            args!(1005, "Kafka", "kafka", 120),
        ))
        .await?;

    let rs = client.execute("SELECT * FROM avatars").await?;

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
    generate_tables().await?;
    // seed().await?;
    Ok(())
}
