use chrono::{DateTime, Local};

use crate::BarModule;


pub struct Clock {
    pub clock_format: &'static str,
    pub refresh_rate: u32,
}

impl BarModule for Clock {
    fn get_value(&self) -> String {
        let date: DateTime<Local> = Local::now();
        date.format(&self.clock_format).to_string()
    }
    fn get_timer(&self) -> u32 {
        self.refresh_rate
    }
}
