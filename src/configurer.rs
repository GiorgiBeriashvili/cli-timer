use std::{
    env,
    fs::{self, OpenOptions},
    path::PathBuf,
};

use crate::logger;

pub struct Configuration {
    pub current_directory: PathBuf,
    pub configuration_directory: PathBuf,
    pub directory_name: &'static str,
    pub file_name: &'static str,
}

pub fn init(configuration: &Configuration, logger: bool) {
    let mut path = PathBuf::new();
    path.push(&configuration.configuration_directory);
    path.push(&configuration.directory_name);

    let application_directory = path.clone();

    path.push(&configuration.file_name);

    env::set_current_dir(&configuration.configuration_directory).unwrap();

    if fs::create_dir(&configuration.directory_name).is_err() {
        if logger::status(logger) {
            env::set_current_dir(&application_directory).unwrap();

            log::trace!("Application's configuration directory already exists. Moving on.");
        }
    } else {
        env::set_current_dir(&configuration.configuration_directory).unwrap();

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
        }
    }

    env::set_current_dir(&configuration.current_directory).unwrap();
}
