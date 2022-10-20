use crate::daemon::daemonize;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use std::{env, thread};

pub fn timer(duration_min: u32) {
    println!("Starting timer, the duration is: {} min(s).", duration_min);

    daemonize().unwrap();

    thread::sleep(duration_from_min(duration_min));

    #[allow(deprecated)]
    let path = build_config_path(env::home_dir().unwrap());

    Command::new("./finished")
        .current_dir(path)
        .output()
        .unwrap();
}

fn build_config_path(home_path: PathBuf) -> PathBuf {
    [home_path, PathBuf::from(".config/timer-cli")]
        .iter()
        .collect()
}

fn duration_from_min(duration_min: u32) -> Duration {
    Duration::from_secs((duration_min * 60).into())
}

#[test]
fn build_finished_path_() {
    let result = build_config_path(PathBuf::from("/home/notation"));
    assert_eq!(result, PathBuf::from("/home/notation/.config/timer-cli"));
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
