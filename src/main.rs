use structopt::StructOpt;
use chrono::prelude::*;
use std::{thread, time};

#[derive(Debug, StructOpt)]
#[structopt(name = "cli-timer", about = "Program used to set a timer.")]
struct Timer {
    /// Turns the timer on or off
    #[structopt(short = "s", long = "status", default_value = "on")]
    status: String,

    /// Sets the timer duration
    #[structopt(short = "d", long = "duration", default_value = "0")]
    duration: u64,

    /// Sets the frequency in seconds
    #[structopt(short = "f", long = "frequency", default_value = "1")]
    frequency: u64,

    /// Sets the time zone (Local, UTC)
    #[structopt(short = "t", long = "timezone", default_value = "local")]
    timezone: String
}

fn main() {
    const FINALE: time::Duration = time::Duration::from_secs(1); // One second pause before the program finishes running
    let mut timer = Timer::from_args();
    let mut execution_time = String::new();
    let mut finish_time = String::new();
    let frequency = time::Duration::from_secs(timer.frequency);

    if timer.status == "on" {
        if timer.timezone.to_lowercase() == "utc" {
            execution_time = Utc::now().to_string();
        }
        else if timer.timezone.to_lowercase() == "local" {
            execution_time = Local::now().to_string();
        };

        println!("Execution time: {}", execution_time);

        let timezone: Vec<&str> = execution_time.as_str().split(" ").collect();
        let timezone = timezone[2];

        if timer.duration != 0 {
            let now = time::Instant::now();

            while timer.duration != 0 {
                println!("{}", timer.duration);
                thread::sleep(frequency);
                timer.duration -= 1;
            }

            if timezone == "UTC" {
                finish_time = Utc::now().to_string();
            }
            else if timezone.starts_with("+") || timezone.starts_with("-") {
                finish_time = Local::now().to_string();
            }

            println!("Finish time: +{} seconds ({})", now.elapsed().as_secs(), finish_time);
        }
        else if timer.duration == 0 {
            println!("\nDuration unspecified. Enter \"cli-timer -d <duration>\" to specify the duration or \"cli-timer -h\" to see documentation.");
        }
    }
    thread::sleep(FINALE);
}
