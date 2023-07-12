use super::types::TextMap;
use crate::{handler::error::WorkerError, routes::honkai::mhy_api::internal::impls::DbData};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct TextHash {
    #[serde(alias = "Hash")]
    pub hash: i64,
}

impl TextHash {
    pub fn get_stable_hash(hash: &str) -> i32 {
        let mut hash1: i32 = 5381;
        let mut hash2: i32 = hash1;

        let mut i = 0;
        while i < hash.len() && hash.as_bytes()[i] as char != '\0' {
            hash1 = ((hash1 << 5).wrapping_add(hash1)) ^ hash.as_bytes()[i] as i32;
            if i == hash.len() - 1 || hash.as_bytes()[i + 1] as char == '\0' {
                break;
            }
            hash2 = ((hash2 << 5).wrapping_add(hash2)) ^ hash.as_bytes()[i + 1] as i32;
            i += 2;
        }

        hash1.wrapping_add(hash2.wrapping_mul(1566083941))
    }

    pub fn read_from_textmap(
        &self,
        text_map: &HashMap<String, String>,
    ) -> Result<String, WorkerError> {
        let value = text_map.get(&self.hash.to_string()).cloned();
        value.map_or(Ok(String::new()), |v| Ok(v))
    }

    pub async fn async_read_from_textmap(&self) -> Result<String, WorkerError> {
        let text_map: HashMap<String, String> = TextMap::read().await?;

        let value = text_map.get(&self.hash.to_string()).cloned();
        value.map_or(Ok(String::new()), |v| Ok(v))
    }
}

#[cfg(test)]
mod tests {
    use super::TextHash;

    #[test]
    fn arst() {
        let t = TextHash::get_stable_hash("371857150");
        dbg!(t);
    }
}
