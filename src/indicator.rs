use pbr::ProgressBar;
use std::time;
use termcolor::Color;

use crate::{color, pattern_matcher::IsIn};

pub fn display(indicator: &str, colored: bool, mut duration: u64, frequency: time::Duration) {
    if indicator.to_lowercase().is_in("numeric") {
        while duration != 0 {
            println!("{}", duration);
            spin_sleep::sleep(frequency);
            duration -= 1;
        }
    } else if indicator.to_lowercase().is_in("graphic") {
        let mut progress_bar = ProgressBar::new(duration);
        progress_bar.format("[=> ]");
        progress_bar.show_speed = false;
        progress_bar.show_time_left = false;

        while duration != 0 {
            progress_bar.inc();
            spin_sleep::sleep(frequency);
            duration -= 1;
        }

        println!();
    } else {
        color::apply_color(
            colored,
            "\nUnsupported indicator. Running through default indicator instead.\n".to_string(),
            Color::Red,
        );

        while duration != 0 {
            println!("{}", duration);
            spin_sleep::sleep(frequency);
            duration -= 1;
        }

        println!();
    }
}
