use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn cancel(time_entry_path: &PathBuf) -> Result<(), &'static str> {
    if Path::new(time_entry_path).exists() {
        fs::remove_file(time_entry_path).unwrap();
        println!("Timer has now been canceled.");
        Ok(())
    } else {
        Err("Cannot cancel, no running timer.")
    }
}
