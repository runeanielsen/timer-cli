use crate::daemon::daemonize;
use crate::duration_mins::DurationMins;
use crate::unix_epoch::UnixEpoch;
use std::error::Error;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime};
use std::{fs, thread};

pub fn start(
    duration: Duration,
    time_entry_path: &PathBuf,
    finished_path: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    if Path::new(time_entry_path).exists() {
        Err("Could not start timer, another timer is already running.")?;
    }

    let end_time_unix_epoch = SystemTime::now().unix_epoch() + duration.as_secs();

    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(time_entry_path)?
        .write_all(end_time_unix_epoch.to_string().as_bytes())?;

    println!(
        "Starting timer, the duration is: {} min(s).",
        duration.as_mins()
    );

    daemonize()?;

    thread::sleep(duration);

    if Path::new(time_entry_path).exists() {
        let time_entry_unix_epoch: u64 = fs::read_to_string(time_entry_path)?.parse()?;

        if time_entry_unix_epoch == end_time_unix_epoch {
            Command::new(finished_path).output()?;
            // If the
            match fs::remove_file(time_entry_path) {
                Ok(()) => Ok(()),
                Err(error) => {
                    match error.kind() {
                        // Do not care if it is not found, it could have been deleted
                        // by the default itself, and won't result in the program
                        // behaving in a bad way.
                        std::io::ErrorKind::NotFound => Ok(()),
                        // This could be permission denied or delete operation interruped.
                        // There is no simple way to handle this.
                        _ => Err(error),
                    }
                }
            }?
        }
    }

    Ok(())
}
