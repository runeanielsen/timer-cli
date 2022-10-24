use std::{fs, path::Path};

use crate::config::Config;

pub fn cancel(config: &Config) {
    if Path::new(&config.time_entry_path).exists() {
        fs::remove_file(&config.time_entry_path).unwrap();
        println!("Timer has now been canceled.");
    } else {
        println!("Cannot cancel, no running timer.");
    }
}
