use crate::BarModule;

pub struct Text {
    pub text: &'static str,
}

impl BarModule for Text {
    fn get_value(&self) -> String {
        self.text.to_string()
    }
    fn get_timer(&self) -> u32 {
        u32::MAX    
    }
}
