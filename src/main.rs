use chrono::Local;
use std::env;
use std::fs;

fn battery_capacity(battery: &str) -> String {
    let path = format!("/sys/class/power_supply/{}/capacity", battery);
    let result = fs::read_to_string(path)
        .expect("Capacity for specified battery could not be read")
        .trim()
        .to_string();

    result
}

fn battery_status(battery: &str) -> String {
    let path = format!("/sys/class/power_supply/{}/status", battery);
    let result = fs::read_to_string(path)
        .expect("Status for specified battery could not be read")
        .trim()
        .to_string();

    result
}

fn datetime() -> String {
    let now = Local::now();
    let result = format!("{}", now.format("%a %d %b [%H:%M]"));

    result
}

fn main() {
    let mut results: Vec<String> = Vec::new();
    for argument in env::args().skip(1) {
        match argument.as_ref() {
            "-bc" => results.push(battery_capacity("BAT1")),
            "-bs" => results.push(battery_status("BAT1")),
            "-dt" => results.push(datetime()),
            _ => println!("Unrecognised option {}", argument),
        }
    }

    println!("{}", results.join(" | "));
}
