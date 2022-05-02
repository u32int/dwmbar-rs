use std::{thread, time};
use std::process::Command;

mod modules;
use modules::*;

// -- General settings -- 
static BAR_REFRESH_RATE_MILIS: u64 = 1000; // Bar refresh rate in milliseconds.

fn main() {
    // module definitions
    // refresh_rate usually refers to how many bar refreshes (by default - 1000ms so seconds)
    // should happen before updating the value.
    let modules: Vec<&dyn BarModule> = vec!{
        &Text { text: "dwmbar" },
        &Mem {
            format: "{used}",
            refresh_rate: 5,
            unit: MemoryUnit::MB,
        },
        &Clock {
            format: "%m-%d %H:%M",
            refresh_rate: 1,
        },
   };

    // Define your separator here (it will be inserted between modules, optional)
    // for no separator set it to ""
    let separator = " ";

    // Build the cache of last known values
    let mut bar_cache: Vec<String> = Vec::new();
    for _ in &modules { bar_cache.push("".to_owned()); }

    let mut timer = 0;
    loop {
        // get new values
        let values = get_values(&modules, timer);
        // replace values in cache with new values if changed
        for i in 0..bar_cache.len() {
            if values[i] != bar_cache[i] && values[i] != "" {
                bar_cache[i] = values[i].clone();
            }
        }
        // convert bar_cache: Vec<String> -> bar: String
        let bar = vec_to_string(&bar_cache, Some(separator));
        // set the bar
        match Command::new("xsetroot").arg("-name").arg(bar.as_str()).spawn() {
            Ok(mut child) => child.wait().expect("process already over."),
            Err(e) => {
                println!("Something went wrong with cmd\nxsetroot -name {}\n", &bar);
                panic!("{}", e);
            }
        };
        thread::sleep(time::Duration::from_millis(BAR_REFRESH_RATE_MILIS));
        timer += 1;
    }
}

fn vec_to_string(cache: &Vec<String>, separator: Option<&'static str>) -> String {
    let mut result = String::new();
    for elem in cache  {
        result.push_str(elem.as_str());
        if separator.is_some() && elem != cache.last().expect("Error: vector has no last element") {
            result.push_str(separator.unwrap());
        }
    }
    return result;
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
    return constructed;
}
