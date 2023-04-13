use std::str::FromStr;

use axum::{http::StatusCode, routing::post, Json, Router};
use base64::{engine, Engine};
use gray_matter::{engine::YAML, Matter};
use serde::{Deserialize, Serialize};
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

async fn parse_mdx(Json(payload): Json<MdxPayload>) -> Result<String, StatusCode> {
    // data:text/markdown;base64,LS0tCnRpdGxlOiBUZXN0IHRpdGxlCmRlc2NyaXB0aW9uOiBUZXN0IGRlc2NyaXB0aW9uCi0tLQoKVGhpcyBpcyBhIGR1bW15IGZpbGUKCmBgYHJ1c3QgZmlsZW5hbWU9InNyYy9tYWluLnJzIgpmbiBtYWluKCkgewogICAgcHJpbnRsbiEoIkhlbGxvIFdvcmxkIik7Cn0KYGBgCg==
    // split function () -> (file_type, encoder, encoded_data)
    // meta and content split
    let index = &payload.file_data.find(',');
    match index {
        None => Err(StatusCode::BAD_REQUEST),
        Some(index) => {
            // "data:text/markdown;base64,", "base64 data"
            let (meta_chunk, data) = payload.file_data.split_at(*index + 1);
            let meta = meta_chunk.trim_end_matches(',').trim_start_matches("data:");
            match meta.find(';') {
                None => Err(StatusCode::BAD_REQUEST),
                Some(metaindex) => {
                    let (file_type, decoder) = meta.split_at(metaindex);
                    let decoder = decoder.trim_start_matches(';');
                    parse_wrapper(Decoder::from_str(decoder).unwrap(), data.to_owned())
                        .map_err(|_| StatusCode::BAD_REQUEST)
                }
            }
        }
    }
}

#[derive(Serialize, Debug)]
enum WorkerError {
    ParseError,
    ComputationError,
}

#[derive(Serialize, Deserialize)]
struct DecodedDataForm {
    title: String,
    description: String,
    content: String,
}

fn parse_wrapper(decoder: Decoder, data: String) -> Result<String, WorkerError> {
    match decoder {
        Decoder::RawString => Err(WorkerError::ComputationError),
        Decoder::Base64 => {
            let Ok(decoded_data_stream) = engine::general_purpose::STANDARD.decode(data) else {
                return Err(WorkerError::ComputationError);
            };
            let Ok(content_chunk) = String::from_utf8(decoded_data_stream) else {
                return Err(WorkerError::ParseError);
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
                Err(_) => Err(WorkerError::ParseError),
            },
            // no frontmatter found
            None => Err(WorkerError::ParseError),
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
