use chrono::{Local, Utc};

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
