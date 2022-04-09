use std::process::Command;

static UPDATECMD: &'static str = "checkupdates";  // Arch and arch based, requires pacman-contrib

pub fn checkupdates() -> String {
    let cmdout = Command::new(UPDATECMD)
        .output().expect("checkupdates failed");
    let stderr = String::from_utf8_lossy(&cmdout.stderr);
    if stderr.len() > 0 {
        return "err".to_string()
    } else {
        let update_number = String::from_utf8_lossy(&cmdout.stdout).lines().count().to_string();
        return update_number;
    }
}
