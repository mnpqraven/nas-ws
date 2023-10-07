use crate::handler::error::WorkerError;
use async_trait::async_trait;

#[async_trait]
pub trait DbAction {
    async fn seed() -> Result<(), WorkerError>;
}
