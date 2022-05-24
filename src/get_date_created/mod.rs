extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
pub fn get_date_created(path: &str) -> String {
    let metadata = std::fs::metadata(path).unwrap();
    let created = metadata.created().unwrap();
    let datetime: DateTime<Utc> = created.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
