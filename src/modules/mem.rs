use std::fs;
use std::collections::HashMap;

pub fn used() -> String {
    let mem_info_fs = fs::read_to_string("/proc/meminfo").unwrap();
    let lines: Vec<&str> = mem_info_fs.lines().collect();
    let mut meminfo: HashMap<&str, u64> = HashMap::new();

    // Generate a HashMap from the file /proc/meminfo so that we can easily access
    // the information using key value
    for line in lines {
        let mut line_split = line.split_whitespace();
        let key = line_split.nth(0).unwrap().strip_suffix(":").unwrap();
        let value = line_split.nth(0).unwrap().parse::<u64>().unwrap();
        meminfo.insert(key, value);
    };

    // as defined in https://gitlab.com/procps-ng/procps/-/blob/newlib/free.c
    let mem_cached_all = meminfo["Cached"] + meminfo["SReclaimable"];
    let used = (meminfo["MemTotal"] - meminfo["Buffers"] - mem_cached_all - meminfo["MemFree"]) / 1024;
    return used.to_string() + "MB";
}
