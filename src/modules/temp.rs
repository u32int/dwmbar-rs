use crate::BarModule;
use std::fs;

pub struct Temp {
    pub format: &'static str,
    pub update_interval: u32,
    // The thermal zone to read the temperature from.
    // Read from /sys/class/thermal/thermal_zone{zone}
    pub zone: u8,
    // Enable color on termperatures higher than Some(val).
    // None disables the colors completely.
    pub color: Option<u32>,
}

const COLOR_WARN: &str = "#e13d48";

impl Temp {
    fn get_temp_at_zone(&self) -> String {
        let temp = fs::read_to_string(format!(
            "{}{}{}",
            "/sys/class/thermal/thermal_zone", self.zone, "/temp"
        )).unwrap_or_else(|_| "Err (check config)".to_string());

        let temp = temp.trim().parse::<u32>().unwrap() / 1000;
	if self.color.is_some() && temp > self.color.unwrap() {
	    format!("^c{}^{}^d^", COLOR_WARN, &temp)
	} else {
	    temp.to_string()
	}
    }
}

impl BarModule for Temp {
    fn eval_keywords(&self, keywords: Vec<&str>) -> Vec<String> {
        let evaled_keywords: Vec<String> = keywords
            .into_iter()
            .map(|keyword| match keyword {
                "temp" => self.get_temp_at_zone(),
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

