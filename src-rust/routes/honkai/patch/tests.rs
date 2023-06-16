use crate::routes::honkai::patch::types::Patch;
use chrono::{TimeZone, Utc};

#[test]
fn boundaries() {
    let today = Utc.with_ymd_and_hms(2023, 6, 30, 1, 9, 48).unwrap();

    let within_patch = Utc.with_ymd_and_hms(2023, 7, 10, 1, 9, 48).unwrap();
    let next_patch = Utc.with_ymd_and_hms(2023, 7, 22, 1, 9, 48).unwrap();

    assert_eq!(Patch::patch_passed_diff(today, within_patch).unwrap(), 0);
    assert_eq!(Patch::patch_passed_diff(today, next_patch).unwrap(), 1);
}

#[test]
fn half_patch_diffing() {
    let today = Utc.with_ymd_and_hms(2023, 6, 11, 1, 9, 48).unwrap();
    let next_patch = Utc.with_ymd_and_hms(2023, 8, 18, 1, 9, 48).unwrap();
    let next_patch2 = Utc.with_ymd_and_hms(2023, 8, 14, 1, 9, 48).unwrap();

    assert_eq!(Patch::half_patch_passed_diff(today, next_patch).unwrap(), 3);
    assert_eq!(
        Patch::half_patch_passed_diff(today, next_patch2).unwrap(),
        3
    );
}
