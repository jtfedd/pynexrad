use chrono::NaiveDate;

/// Creates a date from year, month, and day
pub fn create_date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("date is valid")
}
