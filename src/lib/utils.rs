use chrono::{Date, DateTime, Datelike, Local, NaiveDate, TimeZone, Utc};

pub fn convert_timestamp_to_date(timestamp: i64) -> DateTime<Utc> {
    let now = Local::now();

    let new_timestamp: u32 = timestamp.try_into().unwrap();

    let hours = new_timestamp / 60;
    let minutes = new_timestamp % 60;

    let naive_date =
        NaiveDate::from_ymd(now.year(), now.month(), now.day()).and_hms(hours, minutes, 0);

    DateTime::<Utc>::from_utc(naive_date, Utc)
}

pub fn days_into_year(date: Date<Local>) -> i64 {
    (date - Local.ymd(date.year(), 1, 1)).num_days()
}

pub fn convert_timestamp_to_string(timestamp: i32) -> String {
    let minutes = timestamp / 60;
    let seconds = timestamp % 60;
    format!("{:02}:{:02}", minutes, seconds)
}
