use lazy_static::lazy_static;
use std::fs;

use crate::BarModule;

// Get the kernel version only once, since it is unlikely to change without restarting the bar.
lazy_static! {
    static ref KERNEL_VER: String = {
        let version_file =
            fs::read_to_string("/proc/version").expect("Couldn't read /proc/version");
        version_file.split_whitespace().nth(2).unwrap().to_string()
    };
}

enum TimeUnit {
    Hour,
    Minute,
    Second,
}

pub struct OsInfo {
    pub format: &'static str,
    pub update_interval: u32,
}

impl OsInfo {
    fn uptime(unit: TimeUnit) -> String {
        let uptime_file = fs::read_to_string("/proc/uptime").expect("Couldn't read /proc/uptime");
        let seconds = uptime_file
            .split_whitespace()
            .next()
            .unwrap()
            .parse::<f32>()
            .unwrap() as u32;
        let minutes = seconds / 60;
        let hours = minutes / 60;

        match unit {
            TimeUnit::Hour => hours.to_string(),
            TimeUnit::Minute => (minutes - hours * 60).to_string(),
            TimeUnit::Second => (seconds - minutes * 60).to_string(),
        }
    }
}

impl BarModule for OsInfo {
    fn eval_keywords(&self, keywords: Vec<&str>) -> Vec<String> {
        let evaled_keywords: Vec<String> = keywords
            .into_iter()
            .map(|keyword| match keyword {
                "uptime_hours" => OsInfo::uptime(TimeUnit::Hour),
                "uptime_minutes" => OsInfo::uptime(TimeUnit::Minute),
                "uptime_seconds" => OsInfo::uptime(TimeUnit::Second),
                "kernel_ver" => KERNEL_VER.to_string(),
                _ => keyword.to_string(),
            })
            .collect();

        evaled_keywords
    }

    fn get_value(&self) -> String {
        self.parse_format(self.format.to_string())
    }

    fn get_timer(&self) -> u32 {
        self.update_interval
    }
}
