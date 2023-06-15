use crate::handler::{
    error::{ComputationType, WorkerError},
    FromAxumResponse,
};
use axum::Json;
use chrono::{DateTime, Duration, TimeZone, Utc};
use response_derive::JsonResponse;
use semver::Version;
use serde::Serialize;
use vercel_runtime::{Body, Response, StatusCode};

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Patch's time will always have a 02:00:00 UTC date
pub struct Patch {
    pub name: String,
    pub version: Version,
    pub date_start: DateTime<Utc>,
    pub date_end: DateTime<Utc>,
}
#[derive(Serialize, JsonResponse, Clone, Debug)]
pub struct PatchList {
    patches: Vec<Patch>,
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
        let end_date = start_date + Duration::weeks(6);
        Self {
            name: name.into(),
            version: version.into(),
            date_start: start_date,
            date_end: end_date,
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
}

// name and version in Patch
pub struct PatchInfo(pub String, pub Version);
impl PatchList {
    pub fn calculate_from_base(base_version: Patch, future_patches: Vec<PatchInfo>) -> Self {
        let mut res: Vec<Patch> = vec![base_version.clone()];
        let mut next_start_date = base_version.date_end;
        for PatchInfo(name, version) in future_patches.iter() {
            res.push(Patch::new(name, version.clone(), next_start_date));
            next_start_date += Duration::weeks(6);
        }

        match Utc::now() > base_version.date_start {
            true => {
                res.remove(0);
                Self { patches: res }
            }
            false => Self { patches: res },
        }
    }
}
