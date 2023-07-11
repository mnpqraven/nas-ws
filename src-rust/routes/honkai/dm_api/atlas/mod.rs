use chrono::{DateTime, Datelike, NaiveDateTime, TimeZone, Timelike, Utc};
use serde::{
    de::{self, Visitor},
    Deserializer,
};
use std::{fmt, marker::PhantomData};

pub mod avatar_atlas;
pub mod equipment_atlas;
#[cfg(test)]
mod tests;

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
