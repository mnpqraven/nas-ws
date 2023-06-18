use std::sync::Arc;

use super::categorizing::{DbCharacter, DbCharacterSkill, SkillType};

impl DbCharacter {
    // TODO: handle unwrap
    pub fn skill_ids(&self) -> Arc<[u32]> {
        self.skills
            .iter()
            .map(|e| e.parse::<u32>().unwrap())
            .collect()
    }
}

pub trait Queryable<T, U> {
    fn find_many(&self, by_data: T) -> Arc<[U]>;
}

// impl Queryable for DbCharacter { }
impl Queryable<Arc<[u32]>, DbCharacterSkill> for Arc<[DbCharacterSkill]> {
    fn find_many(&self, skill_ids: Arc<[u32]>) -> Self {
        self.into_iter()
            .filter(|e| skill_ids.contains(&e.id) && e.ttype != SkillType::MazeNormal)
            .cloned()
            .collect()
    }
}
