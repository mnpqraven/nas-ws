use crate::routes::honkai::jade_estimate::types::{DateRange, Server};
use chrono::{DateTime, Duration, TimeZone, Timelike, Utc, Weekday, Datelike};

pub fn get_next_monday(current_date: DateTime<Utc>, server: &Server) -> DateTime<Utc> {
    let mut now = current_date;
    let bound = match now.hour() > server.get_utc_reset_hour() {
        true => now + Duration::days(8),
        false => now + Duration::days(7),
    };
    for date in DateRange(now, bound) {
        if date.weekday() == Weekday::Mon {
            now = Utc
                .with_ymd_and_hms(
                    date.year(),
                    date.month(),
                    date.day(),
                    server.get_utc_reset_hour(),
                    1,
                    0,
                )
                .unwrap();
        }
    }
    now
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, TimeZone, Utc};

    use crate::routes::honkai::{jade_estimate::types::Server, utils::helpers::get_next_monday};

    #[test]
    fn next_monday() {
        let now = Utc.with_ymd_and_hms(2023, 6, 9, 11, 12, 13).unwrap();
        let right = Utc.with_ymd_and_hms(2023, 6, 12, 9, 1, 0).unwrap();
        assert_eq!(get_next_monday(now, &Server::America), right);
    }

    #[test]
    fn monday2() {
        let monday_before = Utc.with_ymd_and_hms(2023, 6, 12, 2, 12, 13).unwrap();
        let monday_after = Utc.with_ymd_and_hms(2023, 6, 12, 14, 12, 13).unwrap();
        let right = Utc.with_ymd_and_hms(2023, 6, 12, 9, 1, 0).unwrap();

        assert_eq!(get_next_monday(monday_before, &Server::America), right);
        assert_eq!(
            get_next_monday(monday_after, &Server::America),
            right + Duration::weeks(1)
        );
    }
}
