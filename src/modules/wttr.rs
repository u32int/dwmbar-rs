use std::process::Command;

use crate::BarModule;

pub struct Wttr {
    pub location: &'static str,
    pub format: &'static str,
    pub wttr_format: &'static str,
    /*
    prefix: %
    VALID WTTR FORMATS (https://github.com/chubin/wttr.in)
    c    Weather condition,
    C    Weather condition textual name,
    x    Weather condition, plain-text symbol,
    h    Humidity,
    t    Temperature (Actual),
    f    Temperature (Feels Like),
    w    Wind,
    l    Location,
    m    Moon phase,
    M    Moon day,
    p    Precipitation (mm/3 hours),
    P    Pressure (hPa),

    D    Dawn*,
    S    Sunrise*,
    z    Zenith*,
    s    Sunset*,
    d    Dusk*,
    T    Current time*,
    Z    Local timezone.
    (*times are shown in the local timezone)
    */
    pub update_interval: u32,
}

impl Wttr {
    fn get_wttr(&self) -> String {
	let location_format = String::from("wttr.in/") + self.location + "\\?format=" + self.wttr_format; 
        let wttr = Command::new("curl").arg(location_format)
            .output().expect("weather check failed!");
        let wttr = String::from_utf8_lossy(&wttr.stdout).to_string();

        wttr
    }
}

impl BarModule for Wttr {
    fn eval_keywords(&self, keywords: Vec<&str>) -> Vec<String> {
	let evaled_keywords: Vec<String> = keywords.into_iter()
	    .map(|keyword| {
		match keyword {
		    "weather" => self.get_wttr(),
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
