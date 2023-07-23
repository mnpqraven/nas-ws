use super::types::TextMap;
use crate::{handler::error::WorkerError, routes::honkai::traits::DbData};
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
impl DbData for TextMap {
    type TUpstream = HashMap<String, String>;
    type TLocal = HashMap<String, String>;

    fn path_data() -> &'static str {
        "TextMap/TextMapEN.json"
    }

    async fn upstream_convert(from: Self::TUpstream) -> Result<Self::TLocal, WorkerError> {
        Ok(from)
    }
}
