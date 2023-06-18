use std::sync::Arc;

use crate::{
    handler::error::{ComputationType, WorkerError},
    routes::honkai::mhy_api::{
        internal::{
            categorizing::{DbCharacter, DbCharacterSkill, Parameter, SkillType},
            constants::{CHARACTER_SKILL_LOCAL, CHARACTER_SKILL_REMOTE},
            get_character_list, get_db_list,
            impls::Queryable,
        },
        types::{character::CharacterElement, shared::AssetPath},
    },
};
use anyhow::Result;
use chrono::{DateTime, Duration, TimeZone, Utc};
use schemars::{
    schema::{InstanceType, SchemaObject},
    JsonSchema,
};
use semver::Version;
use serde::Serialize;
use tracing::info;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Patch's time will always have a 02:00:00 UTC date
pub struct Patch {
    pub name: String,
    pub version: Version,
    pub date_start: DateTime<Utc>,
    pub date_2nd_banner: DateTime<Utc>,
    pub date_end: DateTime<Utc>,
}

#[derive(Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PatchBanner {
    pub character_name: String,            // FK cmp with `name`
    pub icon: Option<AssetPath>,           // FK
    pub element: Option<CharacterElement>, // FK
    pub version: PatchVersion,
    pub date_start: DateTime<Utc>,
    pub date_end: DateTime<Utc>,
    pub skills: Vec<SimpleSkill>,
}
#[derive(Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SimpleSkill {
    pub name: String,
    pub ttype: SkillType,
    pub description: Vec<String>,
    pub params: Vec<Vec<String>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct PatchVersion(pub Version);
impl From<Version> for PatchVersion {
    fn from(value: Version) -> Self {
        Self(value)
    }
}

impl PatchBanner {
    pub async fn from_patches(
        patches: Vec<Patch>,
        banner_info: Vec<(Option<&str>, Option<&str>, Version)>,
    ) -> Result<Vec<Self>> {
        let mut banners: Vec<PatchBanner> = vec![];

        let now = std::time::Instant::now();

        let character_list = get_character_list().await?;

        info!("get_character_list {:.2?}", now.elapsed());

        let skill_db =
            get_db_list::<DbCharacterSkill>(CHARACTER_SKILL_LOCAL, CHARACTER_SKILL_REMOTE).await?;
        let mut patches = patches;
        patches.push(Patch::current());

        for patch in patches.iter() {
            let (char1, char2) = match banner_info
                .iter()
                .find(|(_, _, version)| patch.version.eq(version))
            {
                Some((char1, char2, _)) => (char1.unwrap_or("Unknown"), char2.unwrap_or("Unknown")),
                None => ("Unknown", "Unknown"),
            };

            let fk1 = character_list.iter().find(|e| e.name.eq(char1));
            let fk2 = character_list.iter().find(|e| e.name.eq(char2));
            let split = |fk: Option<&DbCharacter>| match fk {
                Some(x) => (Some(x.icon.clone()), Some(x.element.clone().into())),
                None => (None, None),
            };
            let char_skill = |fk_char: Option<&DbCharacter>| match fk_char {
                Some(character) => skill_db
                    .find_many(character.skill_ids())
                    .iter()
                    .map(|db_character_skill| SimpleSkill {
                        description: db_character_skill
                            .split_description()
                            .iter()
                            .map(|e| e.to_string())
                            .collect::<Vec<String>>(),
                        params: db_character_skill
                            .params
                            .iter()
                            .map(|skill_by_slv: &Parameter| {
                                let sorter = db_character_skill.get_sorted_params_inds();
                                let t = skill_by_slv.sort_by_tuple(sorter);
                                t.iter().map(|e| e.to_string()).collect()
                            })
                            .collect(),
                        name: db_character_skill.name.clone(),
                        ttype: db_character_skill.ttype,
                    })
                    .collect::<Arc<[SimpleSkill]>>(),
                None => vec![].into(),
            };

            let (icon, element) = split(fk1);
            banners.push(PatchBanner {
                character_name: char1.to_string(),
                version: patch.version.clone().into(),
                date_start: patch.date_start,
                date_end: patch.date_2nd_banner,
                icon,
                element,
                skills: char_skill(fk1).to_vec(),
            });
            let (icon, element) = split(fk2);
            banners.push(PatchBanner {
                character_name: char2.to_string(),
                version: patch.version.clone().into(),
                date_start: patch.date_2nd_banner,
                date_end: patch.date_end,
                icon,
                element,
                skills: char_skill(fk2).to_vec(),
            });
        }
        Ok(banners)
    }
}

impl Patch {
    const BASE_1_1: (i32, u32, u32, u32, u32, u32) = (2023, 6, 7, 2, 0, 0);

    pub fn base() -> Self {
        let (year, month, day, hour, min, sec) = Self::BASE_1_1;
        let start_date = Utc
            .with_ymd_and_hms(year, month, day, hour, min, sec)
            .unwrap();
        let version = Version::parse("1.1.0").unwrap();
        Self::new("Galatic Roaming", version, start_date)
    }

    /// get the current patch
    pub fn current() -> Self {
        let mut base = Self::base();
        base.name = String::new();
        while Utc::now() > base.date_end {
            base.next();
        }
        base
    }

    /// get the start date of the 1st banner middle and
    /// the end date of a patch
    pub fn get_boundaries(&self) -> (DateTime<Utc>, DateTime<Utc>, DateTime<Utc>) {
        (
            self.date_start,
            self.date_start + Duration::weeks(3),
            self.date_end,
        )
    }

    pub fn contains(&self, date: DateTime<Utc>) -> bool {
        self.date_start <= date && self.date_end >= date
    }

    /// get the next timeslot of a future patch
    /// WARN: the name and version is not (yet) edited
    pub fn next(&mut self) {
        self.date_start += Duration::weeks(6);
        self.date_end += Duration::weeks(6);
        self.version.minor += 1;
    }

    /// Creates a patch
    /// WARNING: exact hour and min, sec needed
    pub fn new(
        name: impl Into<String>,
        version: impl Into<Version>,
        start_date: DateTime<Utc>,
    ) -> Self {
        let date_end = start_date + Duration::weeks(6);
        let date_2nd_banner = start_date + Duration::weeks(3);
        Self {
            name: name.into(),
            version: version.into(),
            date_start: start_date,
            date_2nd_banner,
            date_end,
        }
    }

    /// Creates a patch around the specified date
    pub fn new_around(date: DateTime<Utc>) -> Self {
        let mut patch = Self::base();
        while patch.date_end < date {
            patch.next()
        }
        patch
    }

    pub fn patch_passed_diff(
        from_date: DateTime<Utc>,
        to_date: DateTime<Utc>,
    ) -> Result<u32, WorkerError> {
        if from_date > to_date {
            return Err(WorkerError::Computation(ComputationType::BadDateComparison));
        }

        // get next bp start date (next patch)
        let mut next_patch = Patch::new_around(from_date);
        next_patch.next();

        let mut amount: u32 = 0;
        while next_patch.date_start < to_date {
            amount += 1;
            next_patch.next()
        }
        Ok(amount)
    }

    /// get the amount of half-patch (3 weeks) spans that passed between 2 dates
    pub fn half_patch_passed_diff(
        from_date: DateTime<Utc>,
        to_date: DateTime<Utc>,
    ) -> Result<u32, WorkerError> {
        if from_date > to_date {
            return Err(WorkerError::Computation(ComputationType::BadDateComparison));
        }
        let (l, m, r) = Patch::current().get_boundaries();
        let mut next_banner_date = match true {
            true if l <= from_date && from_date < m => m,
            true if m <= from_date && from_date < r => r,
            _ => r + Duration::weeks(3),
        };
        let mut amount = 0;
        while next_banner_date < to_date {
            amount += 1;
            next_banner_date += Duration::weeks(3);
        }
        Ok(amount)
    }

    pub fn generate(index: u32, info: Option<Vec<(&str, Version)>>) -> Vec<Self> {
        let mut patches = vec![];
        let mut current = Patch::current();
        let mut next_version = current.version.clone();
        for _ in 0..index {
            next_version.minor += 1;
            let name: String = match info.clone() {
                Some(info) => match info.iter().find(|(_, version)| version.eq(&next_version)) {
                    Some((name, _)) => name.to_string(),
                    None => format!("Patch {}.{}", next_version.major, next_version.minor),
                },
                None => format!("Patch {}.{}", next_version.major, next_version.minor),
            };

            let patch = Patch::new(name, next_version.clone(), current.date_end);
            patches.push(patch);
            current.next();
        }
        patches
    }
}

impl JsonSchema for PatchVersion {
    fn schema_name() -> String {
        "PatchVersion".to_owned()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            ..Default::default()
        }
        .into()
    }
}
