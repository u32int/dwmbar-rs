use std::collections::HashMap;
use std::process::Command;

use crate::BarModule;
use crate::MemoryUnit;

pub struct Disk {
    pub format: &'static str,
    pub dir: &'static str,
    pub unit: MemoryUnit,
    pub refresh_rate: u32,
}

impl Disk {
    fn gen_freemap(&self) -> HashMap<String, String> {
	let mut map: HashMap<String, String> = HashMap::new();

	let dfout = Command::new("df").arg(format!("--block-size={}", self.unit))
	    .arg(self.dir)
	    .output().expect("du command failed!");

	let dfout = String::from_utf8_lossy(&dfout.stdout).to_string();

	let dfout: Vec<&str> = dfout.lines().nth(1).unwrap()
	    .split_whitespace().collect();

	// parse dfout into map
	let keys: Vec<&str> = vec!{"device", "total", "used", "available", "use_percent", "mount_point"};
	for (i, key) in keys.into_iter().enumerate() {
	    map.insert(key.to_string(), dfout[i].to_string());
	}

	map
    }
}

impl BarModule for Disk {
    fn get_func_output(&self, func: String) -> String {
	let freemap = self.gen_freemap();
	
	match func.as_str() {
	    "used" => freemap[&"used".to_string()].clone(),
	    "avail" => freemap[&"available".to_string()].clone(),
	    "total" => freemap[&"total".to_string()].clone(),
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
