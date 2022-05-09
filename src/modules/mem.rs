use std::collections::HashMap;
use std::fs;

use crate::{BarModule, MemoryUnit};

pub struct Mem {
    pub format: &'static str,
    pub update_interval: u32,
    pub unit: MemoryUnit,
}

impl Mem {
    fn get_div(&self) -> u64 {
        match self.unit {
            MemoryUnit::KB => 1,
            MemoryUnit::MB => 1024,
            MemoryUnit::GB => 1048576 // 1024*1024
        }
    }

    fn gen_meminfo(&self) -> HashMap<String, u64> {
        let mem_info_fs = fs::read_to_string("/proc/meminfo").unwrap();
        let lines: Vec<&str> = mem_info_fs.lines().collect();
        let mut meminfo: HashMap<String, u64> = HashMap::new();
        let div: u64 = self.get_div(); 

        // Generate a HashMap from the file /proc/meminfo so that we can easily access
        // the information using key-value
        for line in lines {
            let mut line_split = line.split_whitespace();
            let key = line_split.nth(0).unwrap().strip_suffix(":").unwrap();
            let value = line_split.nth(0).unwrap().parse::<u64>().unwrap();
            meminfo.insert(key.to_string(), value/div);
        };

        // as defined in https://gitlab.com/procps-ng/procps/-/blob/newlib/free.c
	let mem_cached_all = meminfo["Cached"] + meminfo["SReclaimable"];
        let used = meminfo["MemTotal"] - meminfo["Buffers"] - mem_cached_all - meminfo["MemFree"];

	meminfo.insert(String::from("used"), used);
	meminfo.insert(
	    String::from("used_percent"),
	    (used as f32 / meminfo["MemTotal"] as f32 * 100.0) as u64);

        meminfo
    }
}

impl BarModule for Mem {
    fn eval_keywords(&self, keywords: Vec<&str>) -> Vec<String> {
	let meminfo = self.gen_meminfo();

	let evaled_keywords: Vec<String> = keywords.into_iter()
	    .map(|keyword| {
		match keyword {
		    "used" => meminfo["used"].to_string() + self.unit.as_str(),
		    "used_percent" => meminfo["used_percent"].to_string(),
		    "total" => meminfo["MemTotal"].to_string() + self.unit.as_str(),
		    _ => keyword.to_string()
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
