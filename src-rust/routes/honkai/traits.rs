use crate::handler::error::WorkerError;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::path::{Path, PathBuf};
use tracing::info;
use url::Url;

#[cfg(target_os = "windows")]
const PREFIX_LOCAL_TMP: &str = "c:\\tmp\\";
#[cfg(target_os = "linux")]
const PREFIX_LOCAL_TMP: &str = "/tmp/";

const PREFIX_LOCAL_REPO: &str = "../StarRailData";

const PREFIX_REMOTE: &str = "https://raw.githubusercontent.com/Dimbreath/StarRailData/master/";

#[async_trait]
pub trait DbData // <T>
// where
//     T: Serialize + DeserializeOwned + Send + Sync,
{
    type TUpstream: Serialize + DeserializeOwned + Send + Sync;
    type TLocal: Serialize + DeserializeOwned + Send + Sync;

    /// local path name
    fn path_data() -> &'static str;

    /// Try to cache fallback fetch data to disk.
    ///
    /// # Errors
    ///
    /// This function will return an error if fetching data from fallback_url
    /// or writing to disk failed.
    async fn try_write_disk() -> Result<String, WorkerError> {
        let path_suffix = Self::path_data();

        let data = reqwest::get(Self::to_url()?).await?.text().await?;
        std::fs::write(Path::new(PREFIX_LOCAL_TMP).join(path_suffix), data.clone())?;
        Ok(data)
    }

    fn to_url() -> Result<Url, WorkerError> {
        Ok(Url::parse(PREFIX_REMOTE)?.join(Self::path_data())?)
    }
    fn to_local_tmp() -> PathBuf {
        Path::new(PREFIX_LOCAL_TMP).join(Self::path_data())
    }
    fn to_repo() -> PathBuf {
        Path::new(PREFIX_LOCAL_REPO).join(Self::path_data())
    }

    /// read the local file for data, lazily writes from fallback url if not
    /// exist
    /// return hashmap with the db struct's PK as keys
    /// WARN: this will error when used by maps that have multiple depths
    async fn read() -> Result<Self::TLocal, WorkerError> {
        let tmp_path = Self::to_local_tmp();
        let repo_path = Self::to_repo();

        // WARN: this results in runtime crashes if the upstream type is
        // different from normal types
        // (one example being any HashMap that has multiple depths that we
        // previously implemented convert functions for)
        let upstream_data: String = match (repo_path.exists(), tmp_path.exists()) {
            // always read repo data if exist
            (true, _) => std::fs::read_to_string(repo_path)?,
            // fallback to tmp data
            (false, true) => std::fs::read_to_string(tmp_path)?,
            // lazily writes data
            (false, false) => {
                info!("CACHE: MISS");
                let written = Self::try_write_disk().await?;
                info!("CACHE WRITTEN");
                written
            }
        };
        let upstream_parsed: Self::TUpstream = serde_json::from_str(&upstream_data)?;

        let local_data = Self::upstream_convert(upstream_parsed).await?;

        Ok(local_data)
    }

    async fn upstream_convert(from: Self::TUpstream) -> Result<Self::TLocal, WorkerError>;
}
