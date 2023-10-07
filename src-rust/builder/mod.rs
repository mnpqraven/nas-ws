use crate::handler::error::WorkerError;
use async_trait::async_trait;
use libsql_client::{Client, Config};
use serde::Deserialize;

use self::config::EnvConfig;

pub mod config;
pub mod traits;
mod types;

#[derive(Deserialize)]
pub struct MyParams {
    pub id: String,
    pub name: String,
}

#[async_trait]
pub trait AsyncInto<T>: Sized + Send + Sync {
    type Resource;
    /// perform conversion asynchronously
    /// WARN: VERY SLOW ATM, PERFORMANCE OPTIMIZATION NEEDED
    async fn async_into(self) -> Result<T, WorkerError>;

    fn into_using_resource(self, resource: &Self::Resource) -> Result<T, WorkerError>;
}

pub async fn get_db_client() -> Result<Client, WorkerError> {
    let env = EnvConfig::new();

    let client = Client::from_config(Config {
        url: url::Url::parse(&env.db_url)?,
        auth_token: Some(env.db_auth_token),
    })
    .await?;

    Ok(client)
}
