pub fn background(color: &str) -> String {
    return String::from("^b") + color + "^";
}

pub fn foreground(color: &str) -> String {
    return String::from("^c") + color + "^";
}

pub fn reset() -> String {
    return "^d^".to_string();
}
