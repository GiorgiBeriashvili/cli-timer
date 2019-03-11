use pbr::ProgressBar;
use std::{thread, time};

pub fn display(indicator: &str, mut duration: u64, frequency: time::Duration) {
    if indicator.to_lowercase() == "numerical" {
        while duration != 0 {
            println!("{}", duration);
            thread::sleep(frequency);
            duration -= 1;
        }
    } else if indicator.to_lowercase() == "bar" {
        let mut progress_bar = ProgressBar::new(duration);
        progress_bar.format("[=> ]");

        while duration != 0 {
            progress_bar.inc();
            thread::sleep(frequency);
            duration -= 1;
        }

        println!();
    } else {
        println!("Unsupported indicator.");
    }
}
