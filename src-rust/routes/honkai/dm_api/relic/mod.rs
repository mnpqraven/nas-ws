use self::{
    config::{RelicConfig, RelicType},
    main_affix::RelicMainAffixConfig,
    set_config::RelicSetConfig,
    set_skill_config::RelicSetSkillConfig,
    sub_affix::RelicSubAffixConfig,
};
use crate::{
    handler::error::WorkerError,
    routes::{endpoint_types::List, honkai::traits::DbData},
};
use axum::{extract::Path, Json};
use futures::future::try_join_all;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use regex::Regex;
use reqwest::Method;
use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

pub mod config;
pub mod main_affix;
pub mod set_config;
pub mod set_skill_config;
pub mod sub_affix;

pub async fn relic_set(Path(set_id): Path<u32>) -> Result<Json<RelicSetConfig>, WorkerError> {
    let relic_set_db = RelicSetConfig::read().await?;
    let data = relic_set_db
        .get(&set_id)
        .ok_or(WorkerError::NotFound(set_id.to_string()))?;

    Ok(Json(data.clone()))
}

pub async fn relic_set_many() -> Result<Json<List<RelicSetConfig>>, WorkerError> {
    let relic_set_db = RelicSetConfig::read().await?;
    Ok(Json(List::new(relic_set_db.into_values().collect())))
}

pub async fn relic_set_search(
    Path(name): Path<String>,
) -> Result<Json<Option<RelicSetConfig>>, WorkerError> {
    // sanitizes params, only interested in chracters
    let regex = Regex::new("[^a-zA-Z0-9]").unwrap();
    let matcher = SkimMatcherV2::default();

    let relic_set_db = RelicSetConfig::read().await?;

    let relic_name = regex.replace_all(&name, "").to_string();
    let data: Vec<RelicSetConfig> = relic_set_db
        .into_values()
        .filter(|v| {
            let fuzz_result = matcher.fuzzy_match(&v.set_name, &relic_name);
            fuzz_result.is_some()
        })
        .collect();
    if data.is_empty() {
        return Ok(Json(None));
    }
    Ok(Json(data.get(0).cloned()))
}

pub async fn relics_by_set(
    Path(set_id): Path<u32>,
) -> Result<Json<List<RelicConfig>>, WorkerError> {
    let relic_db = RelicConfig::read_splitted_by_setid(set_id).await?;

    let data = relic_db
        .into_iter()
        .filter(|value| value.set_id == set_id)
        .collect();

    Ok(Json(List::new(data)))
}

pub async fn relics_by_set_post(
    Json(params): Json<List<u32>>,
) -> Result<Json<List<RelicConfig>>, WorkerError> {
    let List { list: set_ids } = params;
    println!("should see {:?}", set_ids);
    let data = try_join_all(
        set_ids
            .iter()
            .map(|set_id| RelicConfig::read_splitted_by_setid(*set_id)),
    )
    .await?;
    let mut binding: Vec<RelicConfig> = data.into_iter().flatten().collect();
    // WARN: this swap is needed because DM data have their type flipped
    let flattened_data = binding
        .iter_mut()
        .map(|relic| {
            match relic.ttype {
                RelicType::OBJECT => relic.ttype = RelicType::NECK,
                RelicType::NECK => relic.ttype = RelicType::OBJECT,
                _ => {}
            }
            let t = &*relic;
            t.clone()
        })
        .collect();

    Ok(Json(List::new(flattened_data)))
}

pub async fn substat_spread() -> Result<Json<List<RelicSubAffixConfig>>, WorkerError> {
    let spread_db = RelicSubAffixConfig::read().await?;

    let as_list: Vec<RelicSubAffixConfig> = spread_db.into_values().collect();
    Ok(Json(List::new(as_list)))
}

pub async fn mainstat_spread(
) -> Result<Json<BTreeMap<RelicType, Vec<RelicMainAffixConfig>>>, WorkerError> {
    let spread_db = RelicMainAffixConfig::read().await?;

    Ok(Json(spread_db))
}

pub async fn set_bonus(Path(set_id): Path<u32>) -> Result<Json<RelicSetSkillConfig>, WorkerError> {
    let bonus_db = RelicSetSkillConfig::read().await?;
    let data = bonus_db
        .get(&set_id)
        .cloned()
        .ok_or(WorkerError::NotFound(set_id.to_string()))?;
    Ok(Json(data))
}

pub async fn set_bonus_many(
    method: Method,
    relic_ids: Option<Json<List<u32>>>,
) -> Result<Json<List<RelicSetSkillConfig>>, WorkerError> {
    let bonus_db = RelicSetSkillConfig::read().await?;
    let ids = match (&method, relic_ids) {
        (&Method::POST, Some(Json(List { list }))) => Some(list),
        _ => None,
    };

    let data: Arc<[RelicSetSkillConfig]> = bonus_db
        .iter()
        .filter(|(k, _)| ids.is_none() || ids.as_ref().unwrap().contains(k))
        .map(|(_, v)| v.clone())
        .collect();
    Ok(Json(List::new(data.to_vec())))
}

pub async fn relic_slot_type(
    set_ids: Json<List<u32>>,
) -> Result<Json<HashMap<u32, RelicType>>, WorkerError> {
    let Json(List { list: set_ids }) = set_ids;
    let mut ret: HashMap<u32, RelicType> = HashMap::new();
    for set_id in set_ids {
        let db = RelicConfig::read_splitted_by_setid(set_id).await.unwrap();
        db.iter().for_each(|relic_cfg| {
            ret.insert(relic_cfg.id, relic_cfg.ttype);
        })
    }
    Ok(Json(ret))
}
