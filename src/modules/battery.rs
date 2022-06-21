use super::definitions::*;
use battery::units::{ratio::percent, time::minute};
use battery::Manager;

pub struct Battery {
    pub format: &'static str,
    pub update_interval: u32,
}

impl BarModule for Battery {
    fn eval_keywords(&self, keywords: Vec<&str>) -> Vec<String> {
        let bat = Manager::new().unwrap()
            .batteries().unwrap()
            .nth(0).unwrap()
            .unwrap();

        let evaled_keywords: Vec<String> = keywords
            .into_iter()
            .map(|keyword| match keyword {
                "percentage" => bat.state_of_charge().get::<percent>().round().to_string(),
                "time_to_empty" => {
                    if let Some(tto) = bat.time_to_empty() {
                        tto.get::<minute>().round().to_string()
                    } else {
                        "-".to_string()
                    }
                }
                "time_to_full" => {
                    if let Some(ttf) = bat.time_to_full() {
                        ttf.get::<minute>().round().to_string()
                    } else {
                        "-".to_string()
                    }
                }
                "time_to_full_or_empty" => {
                    use battery::State::*;
                    let state = bat.state();
                    if let Some(time) = match state {
                        Charging => bat.time_to_full(),
                        _ => bat.time_to_empty(),
                    } {
                        time.get::<minute>().round().to_string()
                    } else {
                        "-".to_string()
                    }
                }
                "state" => bat.state().to_string(),
                "state_icon" => {
                    use battery::State::*;
                    match bat.state() {
                        Unknown => "".to_string(),
                        Charging => "".to_string(),
                        Discharging => "".to_string(),
                        Full => "".to_string(),
                        Empty => "".to_string(),
                        _ => "".to_string(),
                    }
                }
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
