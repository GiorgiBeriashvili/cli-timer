use chrono::{Local, Utc};
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::{
    env,
    fs::OpenOptions,
    io::{prelude::*, BufWriter},
    path::{Path, PathBuf},
};

use crate::{configurer::ConfigurationDirectory, pattern_matcher::IsIn, timer};

pub struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let data: String = format!(
                "[{}][{}][{}][{}] - {}\n",
                Local::now().format("%Y-%m-%d"),
                Local::now().format("%H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or_default(),
                record.args()
            );
            let data = data.as_bytes();

            let path = format!("{}.log", env!("CARGO_PKG_NAME"));
            let path = Path::new(&path);
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)
                .unwrap();

            let mut writer = BufWriter::new(&file);
            writer.write_all(data).unwrap();
        }
    }

    fn flush(&self) {}
}

pub fn status(logger: bool) -> bool {
    if logger {
        true
    } else if !logger {
        false
    } else {
        panic!("Could not determine the status of the logger.");
    }
}

pub fn execution(configuration: &ConfigurationDirectory, timer: &mut timer::Timer) -> String {
    let mut execution_time = String::new();
    let mut path = PathBuf::new();
    path.push(&configuration.configuration_directory);
    path.push(&configuration.directory_name);

    let application_directory = path.clone();

    env::set_current_dir(&application_directory).unwrap();

    let unsupported_indicator = "unsupported indicator";

    if timer.indicator.is_in("numeric") {
        timer.indicator = "numeric".to_string();
    } else if timer.indicator.is_in("graphic") {
        timer.indicator = "graphic".to_string();
    } else {
        timer.indicator = unsupported_indicator.to_string();
    };

    if timer.timezone.to_lowercase().is_in("utc") {
        execution_time = Utc::now().to_string();

        if self::status(timer.logger && timer.indicator != unsupported_indicator) {
            log::info!("{}", format!("Executed successfully.\n[DURATION]  = {} SECONDS\n[FREQUENCY] = {} SECONDS\n[TOTAL]     = {} SECONDS\n[INDICATOR] = {}\n[COLORED]   = {}\n[SOUND]     = {}\n[TIMEZONE]  = {}\n[VERSION]   = {}", timer.duration, timer.frequency, timer.total_duration(), timer.indicator.to_uppercase(), timer.colored, timer.sound, "local".to_uppercase(), env!("CARGO_PKG_VERSION")));
        } else {
            log::error!("{}", format!("Execution failed.\n[DURATION]  = {} SECONDS\n[FREQUENCY] = {} SECONDS\n[TOTAL]     = {} SECONDS\n[INDICATOR] = {}\n[COLORED]   = {}\n[SOUND]     = {}\n[TIMEZONE]  = {}\n[VERSION]   = {}", timer.duration, timer.frequency, timer.total_duration(), timer.indicator.to_uppercase(), timer.colored, timer.sound, "local".to_uppercase(), env!("CARGO_PKG_VERSION")))
        }
    } else if timer.timezone.to_lowercase().is_in("local") {
        execution_time = Local::now().to_string();

        if self::status(timer.logger && timer.indicator != unsupported_indicator) {
            log::info!("{}", format!("Executed successfully.\n[DURATION]  = {} SECONDS\n[FREQUENCY] = {} SECONDS\n[TOTAL]     = {} SECONDS\n[INDICATOR] = {}\n[COLORED]   = {}\n[SOUND]     = {}\n[TIMEZONE]  = {}\n[VERSION]   = {}", timer.duration, timer.frequency, timer.total_duration(), timer.indicator.to_uppercase(), timer.colored, timer.sound, "local".to_uppercase(), env!("CARGO_PKG_VERSION")));
        } else {
            log::error!("{}", format!("Execution failed.\n[DURATION]  = {} SECONDS\n[FREQUENCY] = {} SECONDS\n[TOTAL]     = {} SECONDS\n[INDICATOR] = {}\n[COLORED]   = {}\n[SOUND]     = {}\n[TIMEZONE]  = {}\n[VERSION]   = {}", timer.duration, timer.frequency, timer.total_duration(), timer.indicator.to_uppercase(), timer.colored, timer.sound, "local".to_uppercase(), env!("CARGO_PKG_VERSION")))
        }
    };

    execution_time
}

static LOGGER: Logger = Logger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace))
}
