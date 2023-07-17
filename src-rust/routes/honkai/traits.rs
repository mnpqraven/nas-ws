use crate::handler::error::WorkerError;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use tracing::info;

pub trait DbDataLike = Serialize + DeserializeOwned + Send + Sync;

#[async_trait]
pub trait DbData<T>
where
    T: Serialize + DeserializeOwned + Send + Sync,
{
    /// tuple of local path and fallback url to the resource
    fn path_data() -> (&'static str, &'static str);

    /// Try to cache fallback fetch data to disk.
    ///
    /// # Errors
    ///
    /// This function will return an error if fetching data from fallback_url
    /// or writing to disk failed.
    async fn try_write_disk() -> Result<String, WorkerError> {
        let (local_path, fallback_url) = Self::path_data();
        let data = reqwest::get(fallback_url).await?.text().await?;
        std::fs::write(local_path, data.clone())?;
        Ok(data)
    }

    /// read the local file for data, lazily writes from fallback url if not
    /// exist
    /// return hashmap with the db struct's PK as keys
    /// WARN: this will error when used by maps that have multiple depths
    async fn read() -> Result<HashMap<String, T>, WorkerError> {
        let (local_path, _) = Self::path_data();
        let str_data: String = match std::path::Path::new(local_path).exists() {
            true => std::fs::read_to_string(local_path)?,
            // lazily writes data
            false => {
                info!("CACHE: MISS");
                let written = Self::try_write_disk().await?;
                info!("CACHE WRITTEN");
                written
            }
        };
        Ok(serde_json::from_str(&str_data)?)
    }
}
