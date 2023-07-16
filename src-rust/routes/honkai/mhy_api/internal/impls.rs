use super::{
    categorizing::{
        DbCharacter, DbCharacterEidolon, DbCharacterSkill, DbCharacterSkillTree, SkillType,
    },
    constants::*,
};
use crate::routes::honkai::{
    mhy_api::types_parsed::shared::DbAttributeProperty,
    traits::{DbData, DbDataLike},
};
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
