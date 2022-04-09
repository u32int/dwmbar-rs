use std::process::Command;
use std::{thread, time};

mod modules { 
    /*  
    The colors module is optional and gives you the ability to add colors to your bar.
    It only works if you have this patch installed: https://dwm.suckless.org/patches/status2d/ 
    */
    pub mod colors;     

    pub mod clock;
    pub mod cpu;
    pub mod mem;
    pub mod update;
    pub mod wttr;
}

use modules::*;

// -- General settings -- 
static BAR_REFRESH_RATE_MILIS: u64 = 1000; // Bar refresh rate in milliseconds.
static UPDATE_CHECK_DELAY: u32 = 3600; // The number of refresh bar refresh cycles before checking for updates.
static ENABLE_NET_OPERATIONS: bool = false; // Enables or disables all operations requiring internet access. ex. updates, weather.

fn build_bar(persistent: &Data) -> String {
    // Define your modules here
    let modules = [
        "dwmbar".to_string(),
        mem::used(),
        clock::formatted("%Y-%m-%d %H:%M:%S"),
    ];
    // Define your separator here (it will be inserted between modules, optional)
    let separator = " ";


    // Build the final string from modules.
    let mut ret = String::from("");
    for module in &modules {
        ret.push_str(module.as_str());
        if module != &modules[modules.len()-1] {
            ret.push_str(separator);
        }
    }
    return ret;
}

struct Data {
    wttr: String,
    updates: String,
}

impl Data {
    fn update_data(&mut self) {
        if ENABLE_NET_OPERATIONS {
            self.wttr = modules::wttr::update_weather();
            self.updates = modules::update::checkupdates();
            println!("updated");
        }
    }
}

fn main() {
    let mut refresh_counter = 0;
    let mut persistent: Data = Data { wttr: "".into(), updates: "".into() };
    persistent.update_data();

    let mut xsetrootcmd = Command::new("xsetroot");
    loop {
        refresh_counter += 1;
        let bar = build_bar(&persistent);
        if let Ok(mut proc) = xsetrootcmd.arg("-name").arg(bar.as_str()).spawn() {
            // Wait for the process to end. Not doing this leads to "zombie" processes.
            proc.wait().expect("process already over.");
        }
        thread::sleep(time::Duration::from_millis(BAR_REFRESH_RATE_MILIS));
        if refresh_counter > UPDATE_CHECK_DELAY {
            refresh_counter = 0;        
            persistent.update_data();
        }
    }
}
