use crate::handler::error::WorkerError;
use async_trait::async_trait;
use serde::Deserialize;

mod types;
pub mod config;

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
