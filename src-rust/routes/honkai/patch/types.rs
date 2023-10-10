use crate::{
    handler::error::{ComputationType, WorkerError},
    routes::honkai::{
        dm_api::{character::types::AvatarConfig, types::SkillType},
        traits::DbData,
    },
};
use chrono::{DateTime, Duration, TimeZone, Utc};
use schemars::{
    schema::{InstanceType, SchemaObject},
    JsonSchema,
};
use semver::Version;
use serde::Serialize;

#[derive(Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
/// Patch's time will always have a 02:00:00 UTC date
pub struct Patch {
    pub name: String,
    pub version: PatchVersion,
    pub date_start: DateTime<Utc>,
    pub date_2nd_banner: DateTime<Utc>,
    pub date_end: DateTime<Utc>,
}

#[derive(Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PatchBanner {
    pub character_data: Option<AvatarConfig>,
    pub version: PatchVersion,
    pub date_start: DateTime<Utc>,
    pub date_end: DateTime<Utc>,
}

#[derive(Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SimpleSkill {
    pub id: u32,
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
    // TODO: REFACTOR
    pub async fn from_patches(
        patches: Vec<Patch>,
        banner_info: Vec<(Option<&str>, Option<&str>, Version)>,
    ) -> Result<Vec<Self>, WorkerError> {
        let mut banners: Vec<PatchBanner> = vec![];

        let character_list = AvatarConfig::read().await?;

        let mut patches = patches;
        patches.push(Patch::current());

        for patch in patches.iter() {
            let (char1, char2): (Option<&str>, Option<&str>) = match banner_info
                .iter()
                .find(|(_, _, version)| patch.version.0.eq(version))
            {
                Some((char1, char2, _)) => (*char1, *char2),
                None => (None, None),
            };

            let fk1 = character_list
                .iter()
                .find(|(_, e)| char1.is_some() && e.avatar_votag.eq(char1.unwrap()));
            let fk2 = character_list
                .iter()
                .find(|(_, e)| char2.is_some() && e.avatar_votag.eq(char2.unwrap()));

            let char_data1 = match fk1 {
                Some((key, _)) => character_list.get(key).cloned(),
                None => None,
            };
            let char_data2 = match fk2 {
                Some((key, _)) => character_list.get(key).cloned(),
                None => None,
            };
            banners.push(PatchBanner {
                character_data: char_data1,
                version: patch.version.clone(),
                date_start: patch.date_start,
                date_end: patch.date_2nd_banner,
            });
            banners.push(PatchBanner {
                character_data: char_data2,
                version: patch.version.clone(),
                date_start: patch.date_2nd_banner,
                date_end: patch.date_end,
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
        self.date_2nd_banner += Duration::weeks(6);
        self.date_end += Duration::weeks(6);
        self.version.0.minor += 1;
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
            version: PatchVersion(version.into()),
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
        let PatchVersion(mut next_version) = current.version.clone();
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
