use crate::unix_epoch::UnixEpoch;

use std::{fs, path::Path, time::SystemTime};

use crate::config::Config;

pub fn status(config: &Config) {
    if !Path::new(&config.time_entry_path).exists() {
        println!("00:00");
        return;
    }

    let end_unix_epoch: u64 = fs::read_to_string(&config.time_entry_path)
        .unwrap()
        .parse()
        .unwrap();

    let now_unix_epoch = SystemTime::now().unix_epoch();

    let difference = end_unix_epoch - now_unix_epoch;

    println!("{}", format_status(difference));
}

fn format_status(secs: u64) -> String {
    format!("{:0>2}:{:0>2}", secs / 60, secs % 60)
}

#[test]
fn format_status_is_correct_layout() {
    let assertions = vec![
        (0, "00:00"),
        (30, "00:30"),
        (120, "02:00"),
        (130, "02:10"),
        (1200, "20:00"),
        (1230, "20:30"),
    ];

    for assertion in assertions {
        assert_eq!(format_status(assertion.0), assertion.1);
    }
}
