use super::{DecodedDataForm, MdxPayload};
use crate::handler::error::WorkerError;
use axum::{extract::rejection::JsonRejection, Json};
use base64::{engine, Engine};
use gray_matter::{engine::YAML, Matter};
use std::str::FromStr;
use strum::EnumString;

// TODO: to util module
// other way around for exports
pub async fn parse_mdx(
    result: Result<Json<MdxPayload>, JsonRejection>,
) -> Result<Json<DecodedDataForm>, WorkerError> {
    if let Err(err) = result {
        return Err(WorkerError::ParseData(err.body_text()));
    };
    let payload = result.unwrap();
    // data:text/markdown;base64,LS0tCnRpdGxlOiBUZXN0IHRpdGxlCmRlc2NyaXB0aW9uOiBUZXN0IGRlc2NyaXB0aW9uCi0tLQoKVGhpcyBpcyBhIGR1bW15IGZpbGUKCmBgYHJ1c3QgZmlsZW5hbWU9InNyYy9tYWluLnJzIgpmbiBtYWluKCkgewogICAgcHJpbnRsbiEoIkhlbGxvIFdvcmxkIik7Cn0KYGBgCg==
    // split function () -> (file_type, encoder, encoded_data)
    // meta and content split
    if let Some(index) = &payload.file_data.find(',') {
        // "data:text/markdown;base64,", "base64 data"
        let (meta_chunk, data) = payload.file_data.split_at(*index + 1);
        let meta = meta_chunk.trim_end_matches(',').trim_start_matches("data:");
        if let Some(metaindex) = meta.find(';') {
            let (_file_type, binding) = meta.split_at(metaindex);
            let decoder = Decoder::from_str(binding.trim_start_matches(';'))
                .map_err(|_| WorkerError::ParseData("Unsupporetd encoding engine".to_owned()))?;
            Ok(Json(parse_wrapper(decoder, data.to_owned())?))
        } else {
            Err(WorkerError::ParseData(
                "No seperator between file type and encoding engine found".to_owned(),
            ))
        }
    } else {
        Err(WorkerError::ParseData(
            "No seperator between encoding engine and encoded data found".to_owned(),
        ))
    }
}

fn parse_wrapper(decoder: Decoder, data: String) -> Result<DecodedDataForm, WorkerError> {
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
            Ok(data)
        }
    }
}

fn parse_fn_base64(chunk: String) -> Result<DecodedDataForm, WorkerError> {
    let result = Matter::<YAML>::new().parse(&chunk);
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
    use crate::routes::utils::parse_mdx::parse_fn_base64;

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

#[derive(EnumString)]
enum Decoder {
    RawString,
    #[strum(serialize = "base64")]
    Base64,
}
