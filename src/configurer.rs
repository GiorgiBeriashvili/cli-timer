use serde_derive::Serialize;
use std::{
    env,
    fs::{self, OpenOptions},
    path::PathBuf,
};

use crate::logger;

fn write_default_configuration() {
    fs::write("configuration.ron", "Configuration.").unwrap();
}

#[derive(Serialize)]
pub struct DefaultConfiguration {
    pub colored: bool,
    pub duration: u64,
    pub finale: u64,
    pub frequency: u64,
    pub indicator: String,
    pub logger: bool,
    pub sound: bool,
    pub timezone: String,
}

#[derive(Serialize)]
pub struct ConfigurationDirectory {
    pub current_directory: PathBuf,
    pub target_directory: PathBuf,
    pub directory_name: &'static str,
    pub file_name: &'static str,
}

pub fn init(configuration: &ConfigurationDirectory, logger: bool) {
    let mut path = PathBuf::new();
    path.push(&configuration.target_directory);
    path.push(&configuration.directory_name);

    let application_directory = path.clone();

    path.push(&configuration.file_name);

    env::set_current_dir(&configuration.target_directory).unwrap();

    if fs::create_dir(&configuration.directory_name).is_err() {
        if logger::status(logger) {
            env::set_current_dir(&application_directory).unwrap();

            log::trace!("Application's configuration directory already exists. Moving on.");
        }
    } else {
        env::set_current_dir(&configuration.target_directory).unwrap();

        if logger::status(logger) {
            env::set_current_dir(&application_directory).unwrap();

            log::info!("Creating application's configuration directory.");
            log::info!("Creating application's configuration file.");
        }

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path);

        if logger::status(logger) && application_directory.is_dir() {
            log::info!("Successfully created application's configuration directory.");
        }

        if logger::status(logger) && file.is_ok() {
            log::info!("Successfully created application's configuration file.");

            write_default_configuration();
        }
    }

    env::set_current_dir(&configuration.current_directory).unwrap();
}
