use std::fs;

use crate::BarModule;

pub struct Cpu {
    pub format: &'static str,
    pub update_interval: u32,
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
    fn eval_keywords(&self, keywords: Vec<&str>) -> Vec<String> {
	let evaled_keywords: Vec<String> = keywords.into_iter()
	    .map(|keyword| {
		match keyword {
		    "load" => self.average_load(),
		    _ => keyword.to_string(),
		}
	    }).collect();

	evaled_keywords
    }

    fn get_value(&self) -> String {
        self.parse_format(self.format.to_string())
    }

    fn get_timer(&self) -> u32 {
        self.update_interval
    }
}
