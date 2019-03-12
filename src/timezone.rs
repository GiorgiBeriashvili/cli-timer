use chrono::{Local, Utc};

use crate::logger;

pub fn execution(
    timezone: &str,
    logger: bool,
    duration: u64,
    frequency: u64,
    total_duration: u64,
    indicator: &str,
    sound: bool,
) -> String {
    let mut execution_time = String::new();

    if timezone.to_lowercase() == "utc" {
        execution_time = Utc::now().to_string();

        if logger::status(logger) {
            log::info!("{}", format!("execution successful\n[DURATION]  = {} SECONDS\n[FREQUENCY] = {} SECONDS\n[TOTAL]     = {} SECONDS\n[INDICATOR] = {}\n[SOUND]     = {}\n[TIMEZONE]  = {}", duration, frequency, total_duration, indicator.to_uppercase(), sound, timezone.to_uppercase()));
        }
    } else if timezone.to_lowercase() == "local" {
        execution_time = Local::now().to_string();

        if logger::status(logger) {
            log::info!("{}", format!("execution successful\n[DURATION]  = {} SECONDS\n[FREQUENCY] = {} SECONDS\n[TOTAL]     = {} SECONDS\n[INDICATOR] = {}\n[SOUND]     = {}\n[TIMEZONE]  = {}", duration, frequency, total_duration, indicator.to_uppercase(), sound, timezone.to_uppercase()));
        }
    };

    execution_time
}

pub fn get_suffix(execution_time: &str) -> &str {
    let timezone: Vec<&str> = execution_time.split(' ').collect();
    timezone[2]
}

pub fn check_timezone(timezone: &str) -> String {
    let mut finish_time = String::new();

    if timezone == "UTC" {
        finish_time = Utc::now().to_string();
    } else if timezone.starts_with('+') || timezone.starts_with('-') {
        finish_time = Local::now().to_string();
    }

    finish_time
}
