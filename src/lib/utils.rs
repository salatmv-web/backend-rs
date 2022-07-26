use chrono::{Date, DateTime, Datelike, Local, TimeZone, Timelike};

pub fn convert_timestamp_to_date(timestamp: i64) -> Result<DateTime<Local>, String> {
    let now = Local::now();

    let new_timestamp: u32 = timestamp.try_into().unwrap();

    let hours = new_timestamp / 60;
    let minutes = new_timestamp % 60;

    Ok(now
        .with_hour(hours)
        .ok_or("Failed to set hour.")?
        .with_minute(minutes)
        .ok_or("Failed to parse minute")?
        .with_second(0)
        .unwrap())
}
pub fn days_into_year(date: Date<Local>) -> i64 {
    (date - Local.ymd(date.year(), 1, 1)).num_days()
}

pub fn convert_timestamp_to_string(timestamp: i32) -> String {
    let minutes = timestamp / 60;
    let seconds = timestamp % 60;
    format!("{:02}:{:02}", minutes, seconds)
}
