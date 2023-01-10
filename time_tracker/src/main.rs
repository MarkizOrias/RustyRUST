use chrono::{DateTime, Datelike, Local, Timelike};
use serde::Serialize;

#[derive(Serialize, Debug)]
struct DateTimeStamp {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
}

fn main() {
    let now: DateTime<Local> = Local::now();
    let date_timestamp: DateTimeStamp = DateTimeStamp {
        year: now.year(),
        month: now.month(),
        day: now.day(),
        hour: now.hour(),
        minute: now.minute(),
        second: now.second(),
    };

    let serialized = serde_json::to_string(&date_timestamp).unwrap();
    println!("serialized = {}", serialized);
}
