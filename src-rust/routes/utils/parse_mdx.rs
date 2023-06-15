use super::{DecodedDataForm, EncodedFile, MdxPayload};
use crate::handler::error::WorkerError;
use axum::{extract::rejection::JsonRejection, Json};
use base64::{engine, Engine};
use gray_matter::{engine::YAML, Matter};
use std::str::FromStr;
use strum::EnumString;

pub async fn parse_mdx(
    result: Result<Json<MdxPayload>, JsonRejection>,
) -> Result<Json<DecodedDataForm>, WorkerError> {
    match result {
        Ok(Json(decoded)) => {
            let file = parse_header(decoded.file_data)?;
            Ok(Json(parse_with_decoder(file.decoder, file.encoded_data)?))
        }
        Err(err) => Err(WorkerError::ParseData(err.body_text())),
    }
}

fn parse_with_decoder(decoder: Decoder, data: String) -> Result<DecodedDataForm, WorkerError> {
    match decoder {
        Decoder::RawString => Err(WorkerError::ParseData(
            "Raw string decoder not yet implemented".into(),
        )),
        Decoder::Base64 => {
            let Ok(stream) = engine::general_purpose::STANDARD.decode(data) else {
                return Err(WorkerError::ParseData("decoding data failed".into()));
            };
            let Ok(content_chunk) = String::from_utf8(stream) else {
                return Err(WorkerError::ParseData("Decoding data failed".to_owned()));
            };
            let result = Matter::<YAML>::new().parse(&content_chunk);

            Ok(DecodedDataForm {
                title: get_yaml_kv("title", &result)?,
                description: get_yaml_kv("description", &result)?,
                content: result.content,
            })
        }
    }
}

fn get_yaml_kv(key: &str, source: &gray_matter::ParsedEntity) -> Result<String, WorkerError> {
    match source.data.as_ref() {
        Some(yaml_kv) => match yaml_kv[key].as_string() {
            Ok(value) => Ok(value),
            Err(_) => Err(WorkerError::ParseData(format!(
                "field {:?} doesn't exist in {:?}",
                key, yaml_kv
            ))),
        },
        None => Err(WorkerError::ParseData("No Frontmatter found".into())),
    }
}

fn parse_header(chunk: String) -> Result<EncodedFile, WorkerError> {
    if chunk.find(',').is_none() {
        return Err(WorkerError::ParseData(
            "No seperator between encoding engine and encoded data found".to_owned(),
        ));
    }
    // "data:text/markdown;base64,", "base64 data"
    let (meta_chunk, data) = chunk.split_at(chunk.find(',').unwrap() + 1);
    let meta = meta_chunk.trim_end_matches(',').trim_start_matches("data:");
    if let Some(metaindex) = meta.find(';') {
        let (filetype, binding) = meta.split_at(metaindex);
        let decoder = Decoder::from_str(binding.trim_start_matches(';'))
            .map_err(|_| WorkerError::ParseData("Unsupported encoding engine".into()))?;
        Ok(EncodedFile {
            filetype: filetype.into(),
            decoder,
            encoded_data: data.to_owned(),
        })
    } else {
        Err(WorkerError::ParseData(
            "No seperator between file type and encoding engine found".to_owned(),
        ))
    }
}

#[derive(EnumString)]
pub enum Decoder {
    RawString,
    #[strum(serialize = "base64")]
    Base64,
}
