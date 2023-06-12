// we really need to thoroughly unit test all date diffing functions

use chrono::{TimeZone, Utc};

use super::types::{RewardFrequency, Server};

#[test]
fn new_date_diffing() {
    let from_date = Utc.with_ymd_and_hms(2023, 6, 11, 2, 12, 12).unwrap();
    let to_date = Utc.with_ymd_and_hms(2023, 6, 11, 19, 0, 0).unwrap();
    let days = RewardFrequency::Daily
        .get_difference(from_date, to_date, &Server::Asia)
        .unwrap();
    assert_eq!(days, 1);
}

#[test]
fn new_week_diffing() {
    let to_date = Utc.with_ymd_and_hms(2023, 6, 30, 18, 29, 27).unwrap();
    let from_date = Utc.with_ymd_and_hms(2023, 6, 11, 2, 12, 12).unwrap();
    let weeks = RewardFrequency::Weekly
        .get_difference(from_date, to_date, &Server::Asia)
        .unwrap();
    assert_eq!(weeks, 3);
}
