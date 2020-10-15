#![deny(bare_trait_objects)]
use ::std::io::{Write};
use std::fs::OpenOptions;
use chrono::{Utc, Datelike, Timelike};
use crate::kernel::singleton::{Singleton, new_singleton};

pub struct Logger { }

impl Logger {

    pub fn set_driver(driver: Box<dyn Log>) {
        unsafe {
            LOGGER_DRIVER.set_instance(driver);
        }
    }

    // TODO Implement Option Validation for Clean Message

    fn get_driver() -> &'static Box<dyn Log> {
        unsafe {
            return LOGGER_DRIVER.get_instance();
        }
    }

    pub fn info(text: &str) -> () {
        let driver = Logger::get_driver();
        driver.info(text);
    }

    pub fn error(text: &str) -> () {
        let driver = Logger::get_driver();
        driver.error(text);
    }


}

pub struct FileLogger {}

impl FileLogger {
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

impl Log for FileLogger {

    fn info(&self, text: &str) {
        FileLogger::write_to_file("storage/logs/application.log", text);
    }

    fn error(&self, text: &str) {
        FileLogger::write_to_file("storage/logs/errors.log", text);
    }

}

pub trait Log {

    fn info(&self, text: &str);
    fn error(&self, text: &str);

}

static mut LOGGER_DRIVER : Singleton<Box<dyn Log>> = new_singleton();