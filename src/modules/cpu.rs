use std::fs;

pub fn average_load() -> String {
    let load_avg = fs::read_to_string("/proc/loadavg").unwrap();
    let load_avg: Vec<&str> = load_avg.split_whitespace().collect();

    let oneminute_avg = load_avg[0].to_string();

    return oneminute_avg;
}
