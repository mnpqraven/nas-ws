use self::{avatar_atlas::UpstreamAvatarAtlas, equipment_atlas::UpstreamEquipmentAtlas};
use super::equipment_config::equipment_config::EquipmentConfig;
use crate::{
    handler::error::WorkerError,
    routes::{
        endpoint_types::List,
        honkai::{mhy_api::internal::categorizing::DbCharacter, traits::DbData},
    },
};
use axum::Json;
use chrono::{DateTime, Datelike, NaiveDateTime, TimeZone, Timelike, Utc};
use schemars::JsonSchema;
use serde::{
    de::{self, Visitor},
    Deserializer, Serialize,
};
use std::{collections::HashMap, fmt, marker::PhantomData, sync::Arc};

pub mod avatar_atlas;
pub mod equipment_atlas;
#[cfg(test)]
mod tests;

#[derive(Debug, Serialize, Clone, JsonSchema)]
#[serde(rename(serialize = "camelCase"))]
pub struct SignatureAtlas {
    pub char_id: u32,
    pub lc_id: Vec<u32>,
}

pub async fn atlas_list() -> Result<Json<List<SignatureAtlas>>, WorkerError> {
    let chara_db = <DbCharacter as DbData<DbCharacter>>::read().await?;
    let eq_db = <EquipmentConfig as DbData<EquipmentConfig>>::read().await?;

    let equal = |a: &UpstreamAvatarAtlas, b: &UpstreamEquipmentAtlas| {
        let (d1, d2) = (a.gacha_schedule, b.gacha_schedule);
        let date_equal = match (d1, d2) {
            (Some(d1), Some(d2)) => {
                d1.year() == d2.year() && d1.month() == d2.month() && d1.day() == d2.day()
            }
            _ => false,
        };
        let c = chara_db.get(&a.avatar_id.to_string());
        let e = eq_db.get(&b.equipment_id.to_string());
        let rarity_equal = match (c, e) {
            (Some(a), Some(b)) => a.rarity == b.rarity,
            _ => false,
        };
        date_equal && rarity_equal
    };
    // feature ungrouped tuple of charid, weap_id
    // NOTE: make map joining avatar_atlas and equipment_atlas, filter out
    // banner (map A)
    let char_map = <UpstreamAvatarAtlas as DbData<UpstreamAvatarAtlas>>::read().await?;
    let char_map: HashMap<String, UpstreamAvatarAtlas> = char_map
        .into_iter()
        .filter(|(_, v)| v.gacha_schedule.is_some())
        .collect();

    let eq_map = <UpstreamEquipmentAtlas as DbData<UpstreamEquipmentAtlas>>::read().await?;
    let eq_map: HashMap<String, UpstreamEquipmentAtlas> = eq_map
        .into_iter()
        .filter(|(_, v)| v.gacha_schedule.is_some())
        .collect();

    let eq_map_arced = Arc::new(eq_map);
    let char_map_arced = Arc::new(char_map);

    let banner_feature_pair: Arc<[(u32, Vec<u32>)]> = char_map_arced
        .iter()
        .map(|(char_id, char_atlas)| {
            let eq_date = eq_map_arced
                .iter()
                .find(|(_, eq_atlas)| equal(char_atlas, eq_atlas));
            match eq_date {
                Some((eq_id, _)) => (
                    char_id.parse::<u32>().unwrap(),
                    vec![eq_id.parse::<u32>().unwrap()],
                ),
                None => (char_id.parse::<u32>().unwrap(), vec![]),
            }
        })
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
            lc_id: v.clone(),
        })
        .collect();

    Ok(Json(List::new(vec)))
}

pub fn serialize_date_string<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
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
