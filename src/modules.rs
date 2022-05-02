use std::collections::HashMap;
use std::process::Command;
use std::fs;
use std::fmt;

use chrono::{DateTime, Local};

pub enum MemoryUnit {
    KB, MB, GB,
}

impl MemoryUnit {
    fn as_str(&self) -> &'static str {
        match *self {
            MemoryUnit::KB => "K",
            MemoryUnit::MB => "M",
            MemoryUnit::GB => "G",
        }
    }
}

impl fmt::Display for MemoryUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rep = match *self {
            MemoryUnit::KB => "K",
            MemoryUnit::MB => "M",
            MemoryUnit::GB => "G",
        };
        write!(f, "{}", rep)
    }
}

pub trait BarModule {
    // get the final value from format
    fn get_value(&self) -> String;
    // get the module refresh rate
    fn get_timer(&self) -> u32;
    // get the output of a specified function (probably using a match statement)
    fn get_func_output(&self, _func: String) -> String { unimplemented!() }
    // parse the supplied format
    fn parse_format(&self, format: String) -> String {
        let mut result = String::new();
        for keyword in get_keywords(format) {
            result += self.get_func_output(keyword).as_str();
        };
        return result;
    }
}

fn get_keywords(format: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for part in format.split(&['{', '}']) {
        result.push(part.to_string());
    }
    return result
}

// MODULE DEFINITIONS
// TODO Move module definitions to separate files

// -- Text --

pub struct Text {
    pub text: &'static str,
}

impl BarModule for Text {
    fn get_value(&self) -> String {
        return self.text.to_string();
    }
    fn get_timer(&self) -> u32 {
        return u32::MAX;    
    }
}

// -- Mem --

pub struct Mem {
    pub format: &'static str,
    pub refresh_rate: u32,
    pub unit: MemoryUnit,
}

impl Mem {
    fn gen_meminfo(&self) -> HashMap<String, u64> {
        let mem_info_fs = fs::read_to_string("/proc/meminfo").unwrap();
        let lines: Vec<&str> = mem_info_fs.lines().collect();
        let mut meminfo: HashMap<String, u64> = HashMap::new();

        // Generate a HashMap from the file /proc/meminfo so that we can easily access
        // the information using key-value
        for line in lines {
            let mut line_split = line.split_whitespace();
            let key = line_split.nth(0).unwrap().strip_suffix(":").unwrap();
            let value = line_split.nth(0).unwrap().parse::<u64>().unwrap();
            meminfo.insert(key.to_string(), value);
        };

        return meminfo;
    }

    fn get_div(&self) -> u64 {
        return match self.unit {
            MemoryUnit::KB => 1,
            MemoryUnit::MB => 1024,
            MemoryUnit::GB => 1048576 // 1024*1024
        };
    }

    fn used(&self, meminfo: &HashMap<String, u64>) -> String {
        let div: u64 = self.get_div(); 
        // as defined in https://gitlab.com/procps-ng/procps/-/blob/newlib/free.c
        let mem_cached_all = meminfo["Cached"] + meminfo["SReclaimable"];
        let used = (meminfo["MemTotal"] - meminfo["Buffers"] - mem_cached_all - meminfo["MemFree"]) / div;
        return used.to_string() + self.unit.as_str();
    }

    fn total(&self, meminfo: &HashMap<String, u64>) -> String {
        let div: u64 = self.get_div(); 
        return (meminfo["MemTotal"] / div).to_string() + self.unit.as_str();
    }
}

impl BarModule for Mem {
    fn get_func_output(&self, func: String) -> String { 
        let meminfo = self.gen_meminfo();

        match func.as_str() {
            "used" => self.used(&meminfo),
            "total" => self.total(&meminfo),
            _ => func,
        }
    }

    fn get_value(&self) -> String {
        return self.parse_format(self.format.to_string());
    }
    fn get_timer(&self) -> u32 {
        return self.refresh_rate;
    }
}

// -- CPU --

pub struct Cpu {
    pub format: &'static str,
    pub refresh_rate: u32,
}

impl Cpu {
    fn average_load(&self) -> String {
        let load_avg = fs::read_to_string("/proc/loadavg").unwrap();
        let load_avg: Vec<&str> = load_avg.split_whitespace().collect();

        let oneminute_avg = load_avg[0].to_string();
        return oneminute_avg;
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
        return self.parse_format(self.format.to_string());
    }

    fn get_timer(&self) -> u32 {
        return self.refresh_rate;
    }
}

// -- Colors --
//

pub struct Color {
    pub background: Option<&'static str>,
    pub foreground: Option<&'static str>,
}

impl Color {
    fn background(color: &str) -> String {
        return String::from("^b") + color + "^";
    }

    fn foreground(color: &str) -> String {
        return String::from("^c") + color + "^";
    }

}

impl BarModule for Color {
    fn get_value(&self) -> String {
        let mut result = String::new();
        if self.background.is_some() {
            result.push_str(
                &Color::background(&self.background.as_ref().unwrap())
            )
        }
        if self.foreground.is_some() {
            result.push_str(
                &Color::foreground(&self.foreground.as_ref().unwrap())
            )
        }
        return result;
    }
    fn get_timer(&self) -> u32 {
        return u32::MAX
    }
}

pub struct ColorReset;

impl BarModule for ColorReset {
    fn get_value(&self) -> String {
        return "^d^".to_string();
    }
    fn get_timer(&self) -> u32 {
        return u32::MAX;
    }
}

// -- Clock -- 

pub struct Clock {
    pub format: &'static str,
    pub refresh_rate: u32,
}

impl BarModule for Clock {
    fn get_value(&self) -> String {
        let date: DateTime<Local> = Local::now();
        return date.format(&self.format).to_string();
    }
    fn get_timer(&self) -> u32 {
        return self.refresh_rate;
    }
}

// -- Updates -- 

pub struct Updates {
    pub format: &'static str,
    // It needs to be a command that outputs all available updates
    // one per line to stdout.
    pub update_cmd: &'static str,
    pub refresh_rate: u32,
}

impl Updates {
    fn get_update_count(&self) -> usize {
        let updates = Command::new(&self.update_cmd)
            .output()
            .expect("failed to get updates!");
        let updates = String::from_utf8_lossy(&updates.stdout);
        let update_count = updates.lines().count();
        return update_count;
    }
}

impl BarModule for Updates {
    fn get_func_output(&self, func: String) -> String { 
        match func.as_str() {
            "count" => self.get_update_count().to_string(),
            _ => func,
        }
    }

    fn get_value(&self) -> String {
        println!("Getting updates..");
        return self.parse_format(self.format.to_string());
    }

    fn get_timer(&self) -> u32 {
        return self.refresh_rate;
    }
}

// -- Wttr --

pub struct Wttr {
    pub location: &'static str,
    pub refresh_rate: u32,
}

impl BarModule for Wttr {
    fn get_value(&self) -> String {
        println!("getting weather..");
        let format = "%C+%t";
        let location_format = String::from("wttr.in/") + self.location + "\\?format=" + format; 
        let wttr = Command::new("curl").arg(location_format)
            .output().expect("weather check failed!");
        let wttr = String::from_utf8_lossy(&wttr.stdout).to_string();
        return wttr;
    }
    fn get_timer(&self) -> u32 {
        return self.refresh_rate;
    }
}
