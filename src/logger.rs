use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use chrono::Local;
use std::{fs::OpenOptions, io::{prelude::*, BufWriter}, path::Path};

pub struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let data: String = format!("[{}][{}][{}][{}] - {}\n", Local::now().format("%Y-%m-%d"), Local::now().format("%H:%M:%S"), record.level(), record.module_path().unwrap_or_default(), record.args());
            let data = data.as_bytes();

            let path = format!("{}.log", env!("CARGO_PKG_NAME"));
            let path = Path::new(&path);
            let file = OpenOptions::new().create(true).append(true).open(&path).unwrap();

            let mut writer = BufWriter::new(&file);
            writer.write_all(data).unwrap();
        }
    }

    fn flush(&self) {}
}

pub fn status(logger: &String) -> bool {
    if logger == "on" {
        true
    }
    else if logger == "off" {
        false
    }
    else {
        panic!("Could not determine the status of the logger.");
    }
}

static LOGGER: Logger = Logger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace))
}