use chrono::prelude::*;
use log;
use std::{
    thread,
    time::{Duration, Instant},
};
use structopt::StructOpt;

mod audio;
mod indicator;
mod logger;

#[derive(Debug, StructOpt)]
struct Timer {
    /// Sets the timer duration
    #[structopt(name = "duration", short, long, default_value = "0")]
    duration: u64,

    /// Sets the frequency in seconds
    #[structopt(name = "frequency", short, long, default_value = "1")]
    frequency: u64,

    /// Sets the indicator of the timer's progress (Numerical, Bar)
    #[structopt(name = "indicator", short, long, default_value = "numerical")]
    indicator: String,

    /// Turns the logger on
    #[structopt(name = "logger", short, long)]
    logger: bool,

    /// Turns the sound on
    #[structopt(name = "sound", short, long)]
    sound: bool,

    /// Sets the time zone (Local, UTC)
    #[structopt(name = "timezone", short, long, default_value = "local")]
    timezone: String,
}

fn main() {
    logger::init().unwrap();
    const FINALE: Duration = Duration::from_secs(1);
    let timer = Timer::from_args();
    let mut execution_time = String::new();
    let mut finish_time = String::new();
    let frequency = Duration::from_secs(timer.frequency);
    let sound_file = include_bytes!("audio/sound.ogg");
    let total_duration = timer.duration * timer.frequency;

    if timer.timezone.to_lowercase() == "utc" {
        execution_time = Utc::now().to_string();

        if logger::status(timer.logger) {
            log::info!("{}", format!("execution successful\n[DURATION]  = {} SECONDS\n[FREQUENCY] = {} SECONDS\n[TOTAL]     = {} SECONDS\n[INDICATOR] = {}\n[SOUND]     = {}\n[TIMEZONE]  = {}", timer.duration, timer.frequency, total_duration, timer.indicator.to_uppercase(), timer.sound, timer.timezone.to_uppercase()));
        }
    } else if timer.timezone.to_lowercase() == "local" {
        execution_time = Local::now().to_string();

        if logger::status(timer.logger) {
            log::info!("{}", format!("execution successful\n[DURATION]  = {} SECONDS\n[FREQUENCY] = {} SECONDS\n[TOTAL]    = {} SECONDS\n[INDICATOR] = {}\n[SOUND]     = {}\n[TIMEZONE]  = {}", timer.duration, timer.frequency, total_duration, timer.indicator.to_uppercase(), timer.sound, timer.timezone.to_uppercase()));
        }
    };

    println!("Execution time: {}", execution_time);

    let timezone: Vec<&str> = execution_time.as_str().split(' ').collect();
    let timezone = timezone[2];

    if timer.duration != 0 {
        let now = Instant::now();

        indicator::display(&timer.indicator, timer.duration, frequency);

        if timezone == "UTC" {
            finish_time = Utc::now().to_string();
        } else if timezone.starts_with('+') || timezone.starts_with('-') {
            finish_time = Local::now().to_string();
        }

        println!(
            "Finish time: +{} seconds ({})",
            now.elapsed().as_secs(),
            finish_time
        );

        if timer.sound {
            audio::play_audio(sound_file);
        }

        if logger::status(timer.logger) {
            log::info!("finish successful\n");
        }
    } else if timer.duration == 0 {
        println!("\nDuration unspecified. Enter \"cli-timer -d <duration>\" to specify the duration or \"cli-timer -h\" to print the help information.");

        if logger::status(timer.logger) {
            log::warn!("duration unspecified\n");
        }
    }
    thread::sleep(FINALE);
}
