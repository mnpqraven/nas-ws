use crate::{
    handler::error::WorkerError, routes::honkai::mhy_api::types_parsed::shared::DbAttributeProperty,
};

use super::{
    categorizing::{
        DbCharacter, DbCharacterEidolon, DbCharacterSkill, DbCharacterSkillTree, SkillType,
    },
    constants::*,
};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashMap, sync::Arc};
use tracing::info;

pub trait DbDataLike = Serialize + DeserializeOwned + Send + Sync;

impl DbCharacter {
    // TODO: handle unwrap
    pub fn skill_ids(&self) -> Arc<[u32]> {
        self.skills
            .iter()
            .map(|e| e.parse::<u32>().unwrap())
            .collect()
    }
}

impl<T: DbDataLike> DbData<T> for DbCharacter {
    fn path_data() -> (&'static str, &'static str) {
        (CHARACTER_LOCAL, CHARACTER_REMOTE)
    }
}

impl<T: DbDataLike> DbData<T> for DbCharacterSkill {
    fn path_data() -> (&'static str, &'static str) {
        (CHARACTER_SKILL_LOCAL, CHARACTER_SKILL_REMOTE)
    }
}

impl<T: DbDataLike> DbData<T> for DbCharacterSkillTree {
    fn path_data() -> (&'static str, &'static str) {
        (CHARACTER_SKILL_TREE_LOCAL, CHARACTER_SKILL_TREE_REMOTE)
    }
}

impl<T: DbDataLike> DbData<T> for DbCharacterEidolon {
    fn path_data() -> (&'static str, &'static str) {
        (CHARACTER_EIDOLON_LOCAL, CHARACTER_EIDOLON_REMOTE)
    }
}

impl<T: DbDataLike> DbData<T> for DbAttributeProperty {
    fn path_data() -> (&'static str, &'static str) {
        (ATTRIBUTE_PROPERTY_LOCAL, ATTRIBUTE_PROPERTY_REMOTE)
    }
}

pub trait Queryable<T, U> {
    fn find_many(&self, by_data: T) -> Arc<[U]>;
}

// impl Queryable for DbCharacter { }
impl Queryable<Arc<[u32]>, DbCharacterSkill> for Arc<[DbCharacterSkill]> {
    fn find_many(&self, skill_ids: Arc<[u32]>) -> Self {
        self.iter()
            .filter(|e| skill_ids.contains(&e.id) && e.ttype != SkillType::MazeNormal)
            .cloned()
            .collect()
    }
}

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
                Self::try_write_disk().await?
            }
        };
        Ok(serde_json::from_str(&str_data)?)
    }
}

#[async_trait]
pub trait MultiDepth<T>
where
    T: Serialize + DeserializeOwned + Send + Sync,
{

    /// read from previous data, merging hashmaps with diff key into 1 large
    /// item
    async fn read_multi_depth<U>(&self, data: T) -> Result<U, WorkerError>;
}
