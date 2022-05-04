use std::fs;

use crate::BarModule;

pub struct Cpu {
    pub format: &'static str,
    pub refresh_rate: u32,
}

impl Cpu {
    fn average_load(&self) -> String {
        let load_avg = fs::read_to_string("/proc/loadavg").unwrap();
        let load_avg: Vec<&str> = load_avg.split_whitespace().collect();

        let oneminute_avg = load_avg[0].to_string();
        oneminute_avg
    }
}

impl BarModule for Cpu {
    fn get_func_output(&self, func: String) -> String { 
        match func.as_str() {
            "load" => self.average_load(),
            _ => func,
        }
    }

    fn get_value(&self) -> String {
        self.parse_format(self.format.to_string())
    }

    fn get_timer(&self) -> u32 {
        self.refresh_rate
    }
}
