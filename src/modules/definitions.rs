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

    // eval passed keywords and swap them for values if valid
    fn eval_keywords(&self, _keywords: Vec<&str>) -> Vec<String> { unimplemented!() }
    // parse the supplied format
    fn parse_format(&self, format: String) -> String {
	let keywords: Vec<&str> = format.split(&['{', '}']).collect();
	let keywords = self.eval_keywords(keywords);
	let mut ret = String::new();
	keywords.into_iter()
	    .for_each(|keyword| ret.push_str(keyword.as_str()));
	ret
    }
}
