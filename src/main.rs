use log;
use std::{
    thread,
    time::{Duration, Instant},
};
use structopt::StructOpt;
use termcolor::Color;

mod audio;
mod color;
mod indicator;
mod logger;
mod timezone;

#[derive(Debug, StructOpt)]
struct Timer {
    /// Turns the colored output on
    #[structopt(name = "color", short, long)]
    colored: bool,

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
    let frequency = Duration::from_secs(timer.frequency);
    let sound_file = include_bytes!("audio/sound.ogg");
    let total_duration = timer.duration * timer.frequency;

    let execution_time = timezone::execution(
        &timer.timezone,
        timer.logger,
        timer.duration,
        timer.frequency,
        total_duration,
        &timer.indicator,
        timer.sound,
    );

    color::apply_color(
        timer.colored,
        format!("Execution time: {}", &execution_time),
        Color::Cyan,
    );

    let timezone_suffix = timezone::get_suffix(&execution_time);

    if timer.duration != 0 {
        let now = Instant::now();

        indicator::display(&timer.indicator, timer.colored, timer.duration, frequency);

        let finish_time = timezone::check_timezone(&timezone_suffix);

        color::apply_color(
            timer.colored,
            format!(
                "Finish time: +{} seconds ({})",
                now.elapsed().as_secs(),
                finish_time
            ),
            Color::Green,
        );

        if timer.sound {
            audio::play_audio(sound_file);
        }

        if logger::status(timer.logger) {
            log::info!("finish successful\n");
        }
    } else if timer.duration == 0 {
        color::apply_color(
            timer.colored,
            "\nDuration unspecified. Enter \"cli-timer -d <duration>\" to specify the duration or \"cli-timer -h\" to print the help information.".to_string(),
            Color::Magenta,
        );

        if logger::status(timer.logger) {
            log::warn!("duration unspecified\n");
        }
    }
    thread::sleep(FINALE);
}
