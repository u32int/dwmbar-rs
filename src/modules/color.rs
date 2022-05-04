use crate::BarModule;

pub struct Color {
    pub background: Option<&'static str>,
    pub foreground: Option<&'static str>,
}

impl Color {
    fn background(color: &str) -> String {
        String::from("^b") + color + "^"
    }

    fn foreground(color: &str) -> String {
        String::from("^c") + color + "^"
    }

}

impl BarModule for Color {
    fn get_value(&self) -> String {
        let mut result = String::new();
        if self.background.is_some() {
            result.push_str(
                &Color::background(&self.background.as_ref().unwrap())
            )
        }
        if self.foreground.is_some() {
            result.push_str(
                &Color::foreground(&self.foreground.as_ref().unwrap())
            )
        }
        result
    }
    fn get_timer(&self) -> u32 {
        u32::MAX
    }
}

pub struct ColorReset;

impl BarModule for ColorReset {
    fn get_value(&self) -> String {
        "^d^".to_string()
    }
    fn get_timer(&self) -> u32 {
        u32::MAX
    }
}
