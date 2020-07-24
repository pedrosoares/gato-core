#![deny(bare_trait_objects)]
use ::std::io::{Write};
use std::fs::OpenOptions;
use chrono::{Utc, Datelike, Timelike};

pub struct Logger { }

impl Logger {

    pub fn info(text: &str) -> () {
        Logger::write_to_file("storage/logs/application.log", text);
    }

    pub fn error(text: &str) -> () {
        Logger::write_to_file("storage/logs/errors.log", text);
    }

    fn write_to_file(file_name: &str, text: &str) -> () {
        let now = Utc::now();
        let date = format!("{}-{:02}-{:02} {:02}:{:02}:{:02}",
           now.year(), now.month(), now.day(), now.hour(), now.minute(), now.second()
        );
        let result = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(file_name);

        if result.is_ok() {
            let mut file = result.unwrap();
            if let Err(e) = writeln!(file, "[{}] {}", date, text.to_string()) {
                eprintln!("Couldn't write to file: {}", e);
            }
        } else {
            let e = result.err();
            if e.is_some() {
                eprintln!("Couldn't write to file: {}", e.unwrap().to_string());
            }
        }
    }
}