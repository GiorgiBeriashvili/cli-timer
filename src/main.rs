use dirs;
use log;
use std::{
    env, thread,
    time::{Duration, Instant},
};
use structopt::StructOpt;
use termcolor::Color;

mod audio;
mod color;
mod configurer;
mod indicator;
mod logger;
mod pattern_matcher;
mod timer;
mod timezone;

fn main() {
    logger::init().unwrap();
    let mut timer = timer::Timer::from_args();

    let configuration = configurer::ConfigurationDirectory {
        current_directory: env::current_dir().unwrap(),
        configuration_directory: dirs::config_dir().unwrap(),
        directory_name: env!("CARGO_PKG_NAME"),
        file_name: "configuration.toml",
    };

    configurer::init(&configuration, timer.logger);

    const FINALE: Duration = Duration::from_secs(1);
    let frequency = Duration::from_secs(timer.frequency);
    let sound_file = include_bytes!("audio/sound.ogg");

    let execution_time = logger::execution(&configuration, &mut timer);

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
            log::info!("Finished successfully.\n");
        }
    } else if timer.duration == 0 {
        color::apply_color(
            timer.colored,
            "\nDuration unspecified. Enter \"cli-timer -d <duration>\" to specify the duration or \"cli-timer -h\" to print the help information.".to_string(),
            Color::Magenta,
        );

        if logger::status(timer.logger) {
            log::warn!("Duration unspecified.\n");
        }
    }

    env::set_current_dir(&configuration.current_directory).unwrap();

    thread::sleep(FINALE);
}
