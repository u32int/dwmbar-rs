use std::process::Command;

static LOCATION: &'static str = "Warsaw";
static FORMAT: &'static str = "%C+%t"; // Read more: https://github.com/chubin/wttr.in

pub fn update_weather() -> String {
    let location_format = String::from("wttr.in/") + LOCATION + "\\?format=" + FORMAT; 
    let wttr = Command::new("curl").arg(location_format)
        .output().expect("weather check failed!");
    let wttr = String::from_utf8_lossy(&wttr.stdout).to_string();
    return wttr;
}
