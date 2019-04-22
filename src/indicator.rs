use pbr::ProgressBar;
use std::{thread, time};
use termcolor::Color;

use crate::color;

trait IsIn {
    fn is_in(&self, string: &str) -> bool;
}

impl IsIn for str {
    fn is_in(&self, string: &str) -> bool {
        string.contains(self)
    }
}

pub fn display(indicator: &str, colored: bool, mut duration: u64, frequency: time::Duration) {
    let numeric = "numeric";
    let graphic = "graphic";

    if indicator.to_lowercase().is_in(numeric) {
        while duration != 0 {
            println!("{}", duration);
            thread::sleep(frequency);
            duration -= 1;
        }
    } else if indicator.to_lowercase().is_in(graphic) {
        let mut progress_bar = ProgressBar::new(duration);
        progress_bar.format("[=> ]");
        progress_bar.show_speed = false;
        progress_bar.show_time_left = false;

        while duration != 0 {
            progress_bar.inc();
            thread::sleep(frequency);
            duration -= 1;
        }

        println!();
    } else {
        color::apply_color(
            colored,
            "\nUnsupported indicator.\n".to_string(),
            Color::Red,
        );
    }
}
