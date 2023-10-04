use libsql_client::Config;

use crate::{builder::config::EnvConfig, handler::error::WorkerError};

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
