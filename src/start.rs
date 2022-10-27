use crate::daemon::daemonize;
use crate::unix_epoch::UnixEpoch;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime};
use std::{fs, thread};

pub fn start(duration_min: u32, time_entry_path: &PathBuf, finished_script_path: &PathBuf) {
    if Path::new(time_entry_path).exists() {
        eprintln!("Could not start timer, timer is already running.");
        return;
    }

    println!("Starting timer, the duration is: {} min(s).", duration_min);

    let timer_duration = duration_from_min(duration_min);
    let end_time_unix_epoch = SystemTime::now().unix_epoch() + timer_duration.as_secs();

    let mut write_file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(time_entry_path)
        .unwrap();

    write_file
        .write_all(end_time_unix_epoch.to_string().as_bytes())
        .unwrap();

    daemonize().unwrap();

    thread::sleep(timer_duration);

    if Path::new(time_entry_path).exists() {
        let time_entry_unix_epoch = fs::read_to_string(time_entry_path)
            .unwrap()
            .parse::<u64>()
            .unwrap();

        if time_entry_unix_epoch == end_time_unix_epoch {
            Command::new(finished_script_path).output().unwrap();
            // We don't care if it fails, because then it has been removed.
            fs::remove_file(time_entry_path).ok();
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
