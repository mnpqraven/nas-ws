use crate::handler::error::WorkerError;
use axum::{routing::post, Json, Router};
use base64::{engine, Engine};
use gray_matter::{engine::YAML, Matter};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::EnumString;

pub fn utils_routes() -> Router {
    Router::new().route("/parseMdx", post(parse_mdx))
}

#[derive(Deserialize)]
struct MdxPayload {
    #[serde(rename = "fileData")]
    file_data: String,
}

#[derive(EnumString)]
enum Decoder {
    RawString,
    #[strum(serialize = "base64")]
    Base64,
}

// TODO: to util module
// other way around for exports
async fn parse_mdx(Json(payload): Json<MdxPayload>) -> Result<String, WorkerError> {
    // data:text/markdown;base64,LS0tCnRpdGxlOiBUZXN0IHRpdGxlCmRlc2NyaXB0aW9uOiBUZXN0IGRlc2NyaXB0aW9uCi0tLQoKVGhpcyBpcyBhIGR1bW15IGZpbGUKCmBgYHJ1c3QgZmlsZW5hbWU9InNyYy9tYWluLnJzIgpmbiBtYWluKCkgewogICAgcHJpbnRsbiEoIkhlbGxvIFdvcmxkIik7Cn0KYGBgCg==
    // split function () -> (file_type, encoder, encoded_data)
    // meta and content split
    let index = &payload.file_data.find(',');
    match index {
        None => Err(WorkerError::ParseData(
            "No seperator between encoding engine and encoded data found".to_owned(),
        )),
        Some(index) => {
            // "data:text/markdown;base64,", "base64 data"
            let (meta_chunk, data) = payload.file_data.split_at(*index + 1);
            let meta = meta_chunk.trim_end_matches(',').trim_start_matches("data:");
            match meta.find(';') {
                None => Err(WorkerError::ParseData(
                    "No seperator between file type and encoding engine found".to_owned(),
                )),
                Some(metaindex) => {
                    let (file_type, binding) = meta.split_at(metaindex);
                    let decoder =
                        Decoder::from_str(binding.trim_start_matches(';')).map_err(|_| {
                            WorkerError::ParseData("Unsupporetd encoding engine".to_owned())
                        })?;
                    parse_wrapper(decoder, data.to_owned())
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct DecodedDataForm {
    title: String,
    description: String,
    content: String,
}

fn parse_wrapper(decoder: Decoder, data: String) -> Result<String, WorkerError> {
    match decoder {
        Decoder::RawString => Err(WorkerError::Computation),
        Decoder::Base64 => {
            let Ok(decoded_data_stream) = engine::general_purpose::STANDARD.decode(data) else {
                return Err(WorkerError::Computation);
            };
            let Ok(content_chunk) = String::from_utf8(decoded_data_stream) else {
                return Err(WorkerError::ParseData("Decoding data failed".to_owned()));
            };
            let data = parse_fn_base64(content_chunk)?;
            // safe unwrap
            Ok(serde_json::to_string(&data).unwrap())
        }
    }
}

fn parse_fn_base64(chunk: String) -> Result<DecodedDataForm, WorkerError> {
    let matter = Matter::<YAML>::new();
    let result = matter.parse(&chunk);
    let kv_value = |key: &str| {
        match result.data.as_ref() {
            Some(yaml_kv) => match yaml_kv[key].as_string() {
                Ok(value) => Ok(value),
                Err(_) => Err(WorkerError::ParseData(format!(
                    "field {:?} doesn't exist in {:?}",
                    key, yaml_kv
                ))),
            },
            // no frontmatter found
            None => Err(WorkerError::ParseData("No Frontmatter found".to_owned())),
        }
    };

    let decoded = DecodedDataForm {
        title: kv_value("title")?,
        description: kv_value("description")?,
        content: result.content,
    };
    Ok(decoded)
}

#[cfg(test)]
mod test {
    use crate::routes::utils::parse_fn_base64;

    const TEST: &str = r#"---
title: Test title
description: Test description
---

This is a dummy file

```rust filename="src/main.rs"
fn main() {
    println!("Hello World");
}
```
"#;

    #[test]
    fn parsing() {
        let decoded = parse_fn_base64(TEST.to_owned()).unwrap();
        assert_eq!(decoded.title, "Test title");
        assert_eq!(decoded.description, "Test description");
    }
}
