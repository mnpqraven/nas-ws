use super::{
    categorizing::{DbCharacter, DbCharacterSkill, SkillType},
    runnables::{DbData, HasPath},
};
use crate::handler::error::WorkerError;
use std::sync::Arc;

impl DbCharacter {
    // TODO: handle unwrap
    pub fn skill_ids(&self) -> Arc<[u32]> {
        self.skills
            .iter()
            .map(|e| e.parse::<u32>().unwrap())
            .collect()
    }
}

impl HasPath for DbCharacter {
    type T = DbCharacter;
    fn path_data() -> (&'static str, &'static str) {
        ("/tmp/characters.json", "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_new/en/characters.json")
    }
}

impl DbData for DbCharacter {
    type TValue = DbCharacter;
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
