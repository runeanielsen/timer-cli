use crate::daemon::daemonize;
use crate::duration_mins::DurationMins;
use crate::unix_epoch::UnixEpoch;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime};
use std::{fs, thread};

pub fn start(duration: Duration, time_entry_path: &PathBuf, finished_path: &PathBuf) {
    if Path::new(time_entry_path).exists() {
        eprintln!("Could not start timer, timer is already running.");
        return;
    }

    let end_time_unix_epoch = SystemTime::now().unix_epoch() + duration.as_secs();

    let mut write_file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(time_entry_path)
        .unwrap();

    write_file
        .write_all(end_time_unix_epoch.to_string().as_bytes())
        .unwrap();

    println!(
        "Starting timer, the duration is: {} min(s).",
        duration.as_mins()
    );

    daemonize().unwrap();

    thread::sleep(duration);

    if Path::new(time_entry_path).exists() {
        let time_entry_unix_epoch = fs::read_to_string(time_entry_path)
            .unwrap()
            .parse::<u64>()
            .unwrap();

        if time_entry_unix_epoch == end_time_unix_epoch {
            Command::new(finished_path).output().unwrap();
            // We don't care if it fails, because then it has been removed.
            fs::remove_file(time_entry_path).ok();
        }
    }
}
