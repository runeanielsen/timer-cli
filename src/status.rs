use crate::unix_epoch::UnixEpoch;

use std::{
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

pub fn status(time_entry_path: &PathBuf) {
    const DEFAULT_DISPLAY_TIME: &str = "00:00";

    if !Path::new(time_entry_path).exists() {
        println!("{}", DEFAULT_DISPLAY_TIME);
        return;
    }

    let end_unix_epoch: u64 = fs::read_to_string(time_entry_path)
        .unwrap()
        .parse()
        .unwrap();

    let now_unix_epoch = SystemTime::now().unix_epoch();

    // After a while has been deleted, it might still have been loaded in memory here
    // to avoid values being displayed is invalid, we make sure that the 'end_unix_epoch'
    // is greater than the 'now_unix_epoch' that way we only display valid values.
    if end_unix_epoch > now_unix_epoch {
        let difference = end_unix_epoch - now_unix_epoch;
        println!("{}", format_status(difference));
    } else {
        println!("{}", DEFAULT_DISPLAY_TIME);
    }
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
