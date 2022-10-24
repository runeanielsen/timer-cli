use crate::config::Config;
use crate::daemon::daemonize;
use crate::time_entry::UnixEpoch;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, SystemTime};
use std::{fs, thread};

pub fn timer(duration_min: u32, config: &Config) {
    if Path::new(&config.time_entry_path).exists() {
        println!("Could not start timer, timer is already running.");
        return;
    }

    println!("Starting timer, the duration is: {} min(s).", duration_min);

    let duration = duration_from_min(duration_min);
    let end_time_unix_epoch = SystemTime::now().unix_epoch() + duration.as_secs();

    let mut write_file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&config.time_entry_path)
        .unwrap();

    write_file
        .write_all(end_time_unix_epoch.to_string().as_bytes())
        .unwrap();

    daemonize().unwrap();

    thread::sleep(duration);

    if Path::new(&config.time_entry_path).exists() {
        let time_entry_unix_epoch = fs::read_to_string(&config.time_entry_path)
            .unwrap()
            .parse::<u64>()
            .unwrap();

        if time_entry_unix_epoch == end_time_unix_epoch {
            Command::new(&config.finished_path).output().unwrap();

            // We don't care if it fails, because then it has been removed.
            fs::remove_file(&config.time_entry_path).ok();
        }
    }
}

fn duration_from_min(duration_min: u32) -> Duration {
    Duration::from_secs((duration_min * 60).into())
}

#[test]
fn duration_should_be_parsed_from_min_to_duration() {
    let assertions = vec![
        (25, Duration::from_secs(25 * 60)),
        (40, Duration::from_secs(40 * 60)),
        (60, Duration::from_secs(60 * 60)),
        (120, Duration::from_secs(120 * 60)),
        (222, Duration::from_secs(222 * 60)),
    ];

    for (param, result) in assertions {
        assert_eq!(duration_from_min(param), result);
    }
}
