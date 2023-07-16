use super::{
    constants::{TEXT_MAP_LOCAL, TEXT_MAP_REMOTE},
    types::TextMap,
};
use crate::routes::honkai::traits::{DbData, DbDataLike};

impl<T: DbDataLike> DbData<T> for TextMap {
    fn path_data() -> (&'static str, &'static str) {
        (TEXT_MAP_LOCAL, TEXT_MAP_REMOTE)
    }
}
