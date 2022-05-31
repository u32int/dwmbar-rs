use std::process::Command;

use crate::BarModule;

pub struct Updates {
    pub format: &'static str,
    // It needs to be a command that outputs all available updates
    // one per line to stdout.
    pub update_cmd: &'static str,
    pub update_interval: u32,
}

impl Updates {
    fn get_update_count(&self) -> usize {
        let updates = Command::new(&self.update_cmd)
            .output()
            .expect("failed to get updates!");
        let updates = String::from_utf8_lossy(&updates.stdout);
        let update_count = updates.lines().count();
        update_count
    }
}

impl BarModule for Updates {
    fn eval_keywords(&self, keywords: Vec<&str>) -> Vec<String> {
        let evaled_keywords: Vec<String> = keywords
            .into_iter()
            .map(|keyword| match keyword {
                "count" => self.get_update_count().to_string(),
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
