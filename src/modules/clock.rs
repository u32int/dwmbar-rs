use chrono::{DateTime, Local};

pub fn formatted(format: &str) -> String {
    let date: DateTime<Local> = Local::now();
    return date.format(format).to_string();
}
