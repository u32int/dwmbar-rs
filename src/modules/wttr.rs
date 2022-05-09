use std::process::Command;

use crate::BarModule;

pub struct Wttr {
    pub location: &'static str,
    pub update_interval: u32,
}

impl BarModule for Wttr {
    fn get_value(&self) -> String {
        println!("getting weather..");
        let format = "%C+%t";
        let location_format = String::from("wttr.in/") + self.location + "\\?format=" + format; 
        let wttr = Command::new("curl").arg(location_format)
            .output().expect("weather check failed!");
        let wttr = String::from_utf8_lossy(&wttr.stdout).to_string();

        wttr
    }
    fn get_timer(&self) -> u32 {
        self.update_interval
    }
}
