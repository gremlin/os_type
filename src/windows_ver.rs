use regex::Regex;
use std::process::Command;

pub struct WindowsVer {
    pub version: Option<String>
}

pub fn retrieve() -> Option<WindowsVer> {
    let output = match Command::new("cmd").arg("/c").arg("ver").output() {
        Ok(o) => o,
        Err(_) => return None
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    Some(parse(stdout.to_string()))
}

pub fn parse(output: String) -> WindowsVer {
//    let version_regex = Regex::new(r"^Microsoft Windows \[Version\s(\d+\.\d+\.\d+)\]$").unwrap();
//    let version_regex = Regex::new(r"^Microsoft Windows \[Version\s(\d+\.\d+\.\d+\.\d+)\]$").unwrap();
//    let version_regex = Regex::new(r"(\d+\.\d+\.\d+\.\d+)").unwrap();
//    let version_regex = Regex::new(r"Microsoft Windows \[Version\s(\d+\.\d+\.\d+\.\d+)\]").unwrap();

//    let distrib_release_regex = Regex::new(r"Release:\s*([\w\.]+)").unwrap();

//    let version_regex = Regex::new(r#"VERSION_ID="([\w\.]+)"#).unwrap();

//    let version_regex = Regex::new(r"release\s([\w\.]+)").unwrap();

//    let product_version_regex = Regex::new(r"ProductVersion:\s(\w+\.\w+\.\w+)").unwrap();
//    let build_number_regex = Regex::new(r"BuildVersion:\s(\w+)").unwrap();

    // Microsoft Windows [Version 10.0.18362.592]
//    let version_regex = Regex::new(r"Microsoft Windows \[Version\s([\d\.]+)\]").unwrap();
    let version_regex = Regex::new(r"^\s*Microsoft\s*Windows\s*\[Version\s*([\w\.\-]+)\s*\]\s*$").unwrap();

    let version = match version_regex.captures_iter(&output).next() {
        Some(m) => {
            match m.get(1) {
                Some(version) => Some(version.as_str().to_owned()),
                None => None
            }
        },
        None => None
    };
    WindowsVer { version: version }
}
