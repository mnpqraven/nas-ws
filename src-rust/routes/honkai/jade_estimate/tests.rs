use super::types::{RewardFrequency, Server};
use chrono::{TimeZone, Utc};

// TODO: we really need to thoroughly unit test all date diffing functions
// TODO: NA server
#[test]
fn date_diffing() {
    let from_date = Utc.with_ymd_and_hms(2023, 6, 11, 2, 12, 12).unwrap();
    let to_date = Utc.with_ymd_and_hms(2023, 6, 11, 19, 0, 0).unwrap();
    let days = RewardFrequency::Daily
        .get_difference(from_date, to_date, &Server::Asia)
        .unwrap();
    assert_eq!(days, 1);
}

#[test]
fn week_diffing() {
    let from_date = Utc.with_ymd_and_hms(2023, 6, 11, 2, 12, 12).unwrap();
    let to_date = Utc.with_ymd_and_hms(2023, 6, 30, 18, 29, 27).unwrap();
    let weeks = RewardFrequency::Weekly
        .get_difference(from_date, to_date, &Server::Asia)
        .unwrap();
    assert_eq!(weeks, 3);
}

#[test]
fn biweek_diffing_asia() {
    let from_date = Utc.with_ymd_and_hms(2023, 6, 11, 2, 12, 12).unwrap();
    let to_date = Utc.with_ymd_and_hms(2023, 6, 30, 18, 29, 27).unwrap();
    let biweeks = RewardFrequency::BiWeekly
        .get_difference(from_date, to_date, &Server::Asia)
        .unwrap();
    assert_eq!(biweeks, 2);

    let from_date = Utc.with_ymd_and_hms(2023, 6, 19, 19, 12, 12).unwrap();
    let biweeks = RewardFrequency::BiWeekly
        .get_difference(from_date, to_date, &Server::Asia)
        .unwrap();
    assert_eq!(biweeks, 1);
}

#[test]
fn biweek_diffing_na() {
    let from_date = Utc.with_ymd_and_hms(2023, 6, 12, 1, 12, 12).unwrap();
    let to_date = Utc.with_ymd_and_hms(2023, 6, 12, 18, 29, 27).unwrap();
    let biweeks = RewardFrequency::BiWeekly
        .get_difference(from_date, to_date, &Server::America)
        .unwrap();
    assert_eq!(biweeks, 1);

    let from_date = Utc.with_ymd_and_hms(2023, 6, 13, 14, 12, 12).unwrap();
    let to_date = Utc.with_ymd_and_hms(2023, 6, 13, 18, 29, 27).unwrap();
    let biweeks = RewardFrequency::BiWeekly
        .get_difference(from_date, to_date, &Server::America)
        .unwrap();
    assert_eq!(biweeks, 0);

    let from_date = Utc.with_ymd_and_hms(2023, 6, 12, 1, 12, 12).unwrap();
    let to_date = Utc.with_ymd_and_hms(2023, 6, 26, 18, 29, 27).unwrap();
    let biweeks = RewardFrequency::BiWeekly
        .get_difference(from_date, to_date, &Server::America)
        .unwrap();
    assert_eq!(biweeks, 2);
}

#[test]
fn month_diffing() {
    let from_date = Utc.with_ymd_and_hms(2023, 6, 30, 9, 29, 27).unwrap();
    let to_date = Utc.with_ymd_and_hms(2023, 8, 1, 18, 29, 27).unwrap();
    let diff = RewardFrequency::Monthly
        .get_difference(from_date, to_date, &Server::Asia)
        .unwrap();
    assert_eq!(diff, 2);
}

#[test]
fn half_patch_diffing() {
    let from_date = Utc.with_ymd_and_hms(2023, 6, 15, 9, 29, 27).unwrap();
    let to_date = Utc.with_ymd_and_hms(2023, 6, 29, 18, 29, 27).unwrap();
    let diff = RewardFrequency::HalfPatch
        .get_difference(from_date, to_date, &Server::Asia)
        .unwrap();
    assert_eq!(diff, 1);
    let to_date = Utc.with_ymd_and_hms(2023, 7, 20, 18, 29, 27).unwrap();
    let diff = RewardFrequency::HalfPatch
        .get_difference(from_date, to_date, &Server::Asia)
        .unwrap();
    assert_eq!(diff, 2);
}

#[test]
fn whole_patch_diffing() {
    let from_date = Utc.with_ymd_and_hms(2023, 6, 15, 9, 29, 27).unwrap();
    let to_date = Utc.with_ymd_and_hms(2023, 7, 29, 18, 29, 27).unwrap();
    let diff = RewardFrequency::WholePatch
        .get_difference(from_date, to_date, &Server::Asia)
        .unwrap();
    assert_eq!(diff, 1);
    let to_date = Utc.with_ymd_and_hms(2023, 9, 1, 18, 29, 27).unwrap();
    let diff = RewardFrequency::WholePatch
        .get_difference(from_date, to_date, &Server::Asia)
        .unwrap();
    assert_eq!(diff, 2);
}
