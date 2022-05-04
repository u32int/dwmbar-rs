use std::fmt;

#[allow(dead_code)]
pub enum MemoryUnit {
    KB, MB, GB,
}

impl MemoryUnit {
    pub fn as_str(&self) -> &'static str {
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
        result
    }
}

pub fn get_keywords(format: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for part in format.split(&['{', '}']) {
        result.push(part.to_string());
    }
    result
}
