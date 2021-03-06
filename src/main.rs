#![allow(unused_imports)]

use std::process::Command;
use std::{process::exit, thread, time};

mod modules;
use modules::{
    battery::Battery,
    clock::Clock,
    color::{Color, ColorReset},
    cpu::Cpu,
    definitions::*,
    disk::Disk,
    mem::Mem,
    osinfo::OsInfo,
    text::Text,
    updates::Updates,
    wttr::Wttr,
    temp::Temp,
};

// -- General settings --
static BAR_REFRESH_RATE_MILIS: u64 = 1000; // Bar refresh rate in milliseconds.

fn main() {
    // -- CONFIGURATION --
    // update_interval = wait for n times BAR_REFRESH_RATE_MILIS before updating
    let modules: Vec<&dyn BarModule> = vec![
        &Text { text: "dwmbar" },
        &Mem {
            format: "{used}",
            update_interval: 5,
            unit: MemoryUnit::MB,
        },
        &Clock {
            clock_format: "%m-%d %H:%M",
            update_interval: 1,
        },
    ];
    // Define your separator here (it will be inserted between modules, optional)
    // for no separator set it to ""
    let separator = " ";

    // -- end CONFIGURATION --

    // Check if another instance of dwmbar is running
    let out = Command::new("ps")
        .arg("x")
        .output()
        .expect("ps command failed.");
    let out = String::from_utf8_lossy(&out.stdout);
    let mut self_detected = false;
    for line in out.lines() {
        if line.contains("dwmbar") {
            if !self_detected {
                self_detected = true;
            } else {
                eprintln!("Another instance of dwmbar is already running!\nExiting.");
                exit(1);
            }
        }
    }

    // Build the cache of last known values
    let mut bar_cache: Vec<String> = Vec::new();
    for _ in &modules {
        bar_cache.push("".to_owned());
    }

    let mut timer = 0;
    loop {
        let values = get_values(&modules, timer);
        // replace values in cache with new values if changed
        for i in 0..bar_cache.len() {
            if values[i] != bar_cache[i] && !values[i].is_empty() {
                bar_cache[i] = values[i].clone();
            }
        }
        // convert bar_cache: Vec<String> -> bar: String
        let bar = vec_to_string(&bar_cache, Some(separator));
        // set the bar
        match Command::new("xsetroot")
            .arg("-name")
            .arg(bar.as_str())
            .spawn()
        {
            Ok(mut child) => child.wait().expect("process already over."),
            Err(e) => {
                eprintln!("Something went wrong with cmd\nxsetroot -name {}\n", &bar);
                panic!("{}", e);
            }
        };
        thread::sleep(time::Duration::from_millis(BAR_REFRESH_RATE_MILIS));
        timer += 1;
    }
}

fn vec_to_string(cache: &Vec<String>, separator: Option<&str>) -> String {
    let mut result = String::new();
    for elem in cache {
        result.push_str(elem.as_str());
        if separator.is_some() && elem != cache.last().expect("Error: vector has no last element") {
            result.push_str(separator.unwrap());
        }
    }
    result
}

// Update module values if timer matches refresh rate
fn get_values(modules: &Vec<&dyn BarModule>, timer: u32) -> Vec<String> {
    let mut constructed: Vec<String> = Vec::new();
    for module in modules {
        if timer % module.get_timer() == 0 {
            constructed.push(module.get_value());
        } else {
            constructed.push("".to_string());
        }
    }
    constructed
}
