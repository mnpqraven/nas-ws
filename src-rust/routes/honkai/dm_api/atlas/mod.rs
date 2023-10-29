use crate::{
    handler::error::WorkerError,
    routes::{
        endpoint_types::List,
        honkai::{dm_api::atlas::avatar_atlas::UpstreamAvatarAtlas, traits::DbData},
    },
};
use axum::Json;
use chrono::{DateTime, Datelike, NaiveDateTime, TimeZone, Timelike, Utc};
use schemars::JsonSchema;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::{collections::HashMap, fmt, marker::PhantomData, sync::Arc};
use tokio::time::Instant;
use tracing::info;

pub mod avatar_atlas;
pub mod equipment_atlas;
pub mod rpc;
#[cfg(test)]
mod tests;

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename(serialize = "camelCase"))]
pub struct SignatureAtlas {
    pub char_id: u32,
    pub lc_ids: Vec<u32>,
}

pub async fn atlas_list() -> Result<Json<List<SignatureAtlas>>, WorkerError> {
    let now = Instant::now();

    let char_map = UpstreamAvatarAtlas::read().await?;
    let char_map: HashMap<u32, UpstreamAvatarAtlas> = char_map
        .into_iter()
        // .filter(|(_, v)| v.gacha_schedule.is_some())
        .collect();

    let char_map_arced = Arc::new(char_map);

    let banner_feature_pair: Arc<[(u32, Vec<u32>)]> = char_map_arced
        .iter()
        .map(|(char_id, _)| (*char_id, vec![]))
        .collect();

    let base_feature_pair: Arc<[(u32, Vec<u32>)]> = Arc::new([
        (1001, vec![21002]),        // march
        (1002, vec![21003]),        // dan heng
        (1003, vec![23000]),        // himeko
        (1004, vec![23004]),        // welt
        (1006, vec![23007, 22000]), // silver wolf
        (1008, vec![21012]),        // arlan
        (1009, vec![21011]),        // asta
        (1013, vec![21006]),        // herta
        (1101, vec![23003]),        // bronya
        (1102, vec![23001]),        // seele
        (1103, vec![21013]),        // serval
        (1104, vec![23005]),        // gepard
        (1105, vec![21000]),        // natasha
        (1106, vec![21001]),        // pela
        (1107, vec![23002]),        // clara
        (1108, vec![21008]),        // sampo
        (1109, vec![21005]),        // hook
        (1209, vec![23012]),        // yq
        (1201, vec![21034]),        // qq
        (1202, vec![21032]),        // tingyun
        (1204, vec![23010]),        // jing yuan
        (1206, vec![21010]),        // sushang
        (1207, vec![21025]),        // yukong
        (1211, vec![23013]),        // bailu
        (1205, vec![23009]),        // blade
        (1005, vec![23006]),        // kafka
        (1111, vec![21015]),        // luka
        (1208, vec![23011]),        // fuxuan
        (1213, vec![23015]),        // danhengil
        (1212, vec![23014]),        // jingliu
        (1112, vec![23016]),        // topaz
    ]);
    let mut base_feature_map: HashMap<u32, Vec<u32>> = HashMap::new();
    // populate
    base_feature_pair.iter().for_each(|(k, v)| {
        base_feature_map.insert(*k, v.to_vec());
    });

    for (k, v) in banner_feature_pair.iter() {
        if let Some(eqs_in_map) = base_feature_map.get_mut(k) {
            eqs_in_map.extend_from_slice(v);
        } else {
            base_feature_map.insert(*k, v.to_vec());
        }
    }

    let vec: Vec<SignatureAtlas> = base_feature_map
        .iter()
        .map(|(k, v)| SignatureAtlas {
            char_id: *k,
            lc_ids: v.clone(),
        })
        .collect();

    info!("/signature_atlas: {:?}", now.elapsed());
    Ok(Json(List::new(vec)))
}

pub fn _serialize_date_string<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    struct OptionDateEmptyNone<S>(PhantomData<S>);
    impl<'de> Visitor<'de> for OptionDateEmptyNone<DateTime<Utc>> {
        type Value = Option<DateTime<Utc>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("any string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match value {
                "" => Ok(None),
                // v => S::from_str(v).map(Some).map_err(de::Error::custom),
                v => NaiveDateTime::parse_from_str(v, "%F  %T")
                    .map(|naive_date| {
                        let utc_date = Utc
                            .with_ymd_and_hms(
                                naive_date.year(),
                                naive_date.month(),
                                naive_date.day(),
                                naive_date.hour(),
                                naive_date.minute(),
                                naive_date.second(),
                            )
                            .unwrap();
                        Some(utc_date)
                    })
                    .map_err(de::Error::custom),
            }
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            dbg!("visit_string");
            match &*value {
                "" => Ok(None),
                // v => S::from_str(v).map(Some).map_err(de::Error::custom),
                v => NaiveDateTime::parse_from_str(v, "%F  %T")
                    .map(|naive_date| {
                        let utc_date = Utc
                            .with_ymd_and_hms(
                                naive_date.year(),
                                naive_date.month(),
                                naive_date.day(),
                                naive_date.hour(),
                                naive_date.minute(),
                                naive_date.second(),
                            )
                            .unwrap();
                        Some(utc_date)
                    })
                    .map_err(de::Error::custom),
            }
        }

        // handles the `null` case
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_any(OptionDateEmptyNone(PhantomData))
}
