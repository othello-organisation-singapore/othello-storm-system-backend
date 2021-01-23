use std::time::{SystemTime, UNIX_EPOCH};

use chrono::NaiveDate;
use mocktopus::macros::mockable;

use crate::errors::ErrorType;

#[mockable]
pub fn get_current_timestamp() -> u64 {
    let start = SystemTime::now();
    start.duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs()
}

pub fn create_date_format(year: i32, month: u32, date: u32) -> NaiveDate {
    NaiveDate::from_ymd(year, month, date)
}

pub fn date_to_string(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

pub fn string_to_date(date: String) -> Result<NaiveDate, ErrorType> {
    match NaiveDate::parse_from_str(&date[..], "%Y-%m-%d") {
        Ok(date) => Ok(date),
        Err(_e) => Err(ErrorType::BadRequestError(String::from("Bad date format.")))
    }
}
