use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn cancel(time_entry_path: &PathBuf) {
    if Path::new(time_entry_path).exists() {
        fs::remove_file(time_entry_path).unwrap();
        println!("Timer has now been canceled.");
    } else {
        eprintln!("Cannot cancel, no running timer.");
    }
}
