use crate::handler::error::WorkerError;

use super::{
    categorizing::{
        DbCharacter, DbCharacterEidolon, DbCharacterSkill, DbCharacterSkillTree, SkillType,
    },
    constants::*,
};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashMap, sync::Arc};

trait DbDataLike = Serialize + DeserializeOwned + Send + Sync;

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
    fn path_data() -> (&'static str, &'static str);

    /// Try to cache fallback fetch data to disk.
    ///
    /// # Errors
    ///
    /// This function will return an error if fetching data from fallback_url
    /// or writing to disk failed.
    async fn try_write_disk(local_path: &str) -> Result<String, WorkerError> {
        let (_, fallback_url) = Self::path_data();
        let data = reqwest::get(fallback_url).await?.text().await?;
        std::fs::write(local_path, data.clone())?;
        Ok(data)
    }

    /// read the local file for data, lazily writes from fallback url if not
    /// exist
    /// return hashmap with the db struct's PK as keys
    async fn read() -> Result<HashMap<String, T>, WorkerError> {
        let (local_path, _) = Self::path_data();
        let str_data: String = match std::path::Path::new(local_path).exists() {
            true => std::fs::read_to_string(local_path)?,
            // lazily writes data
            false => Self::try_write_disk(local_path).await?,
        };
        Ok(serde_json::from_str(&str_data)?)
    }
}
