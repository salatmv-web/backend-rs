use chrono::{Date, Datelike, Local, NaiveDate, NaiveDateTime, TimeZone, Timelike, Utc};

pub fn convert_timestamp_to_date(timestamp: i64) -> NaiveDateTime {
    let now = Local::now();

    let converted = NaiveDateTime::from_timestamp(timestamp, 0);

    NaiveDate::from_ymd(now.year(), now.month(), now.day()).and_hms(
        converted.minute(),
        converted.second(),
        0,
    )
}
//
pub fn days_into_year(date: Date<Utc>) -> i64 {
    (date - Utc.ymd(date.year(), 1, 1)).num_days()
}

// pub fn convert_timestamp_to_string(timestamp: i32) -> String {
//     let minutes = timestamp / 60;
//     let seconds = timestamp % 60;
//     format!("{:02}:{:02}", minutes, seconds)
// }
