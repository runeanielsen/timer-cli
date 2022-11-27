use crate::unix_epoch::UnixEpoch;

use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

pub fn status(time_entry_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let unix_epoch_left = if Path::new(time_entry_path).exists() {
        let end_unix_epoch: u64 = fs::read_to_string(time_entry_path)?.parse()?;
        let now_unix_epoch = SystemTime::now().unix_epoch();

        // It might take a bit before the file is deleted on disk,
        // to avoid negative time being displayed, we make sure that
        // the end time is greater than the current time, if it is not we just
        // use the default value of `u64`.
        unix_epoch_difference(now_unix_epoch, end_unix_epoch).unwrap_or_default()
    } else {
        0
    };

    println!("{}", format_status(unix_epoch_left));

    Ok(())
}

fn unix_epoch_difference(start: u64, end: u64) -> Option<u64> {
    // If the end is greater than or equal to start, we return none
    // since we don't want to underflow/overflow the return value.
    if end >= start {
        Some(end - start)
    } else {
        None
    }
}

fn format_status(secs: u64) -> String {
    format!("{:0>2}:{:0>2}", secs / 60, secs % 60)
}

#[test]
fn calculate_correct_epoch_difference() {
    assert_eq!(unix_epoch_difference(1_669_560_091, 1_669_560_091), Some(0));
    assert_eq!(
        unix_epoch_difference(1_669_560_091, 1_669_560_091 + 10),
        Some(10)
    );
    assert_eq!(
        unix_epoch_difference(1_669_560_091, 1_669_560_091 + 50),
        Some(50)
    );
    assert_eq!(
        unix_epoch_difference(1_669_560_091, 1_669_560_091 + 121),
        Some(121)
    );
    assert_eq!(
        unix_epoch_difference(1_669_560_091, 1_669_560_091 + 9999),
        Some(9999)
    );
}

#[test]
fn epoch_difference_should_be_none_when_end_is_greater_than_start() {
    assert_eq!(
        unix_epoch_difference(1_669_560_091 + 100, 1_669_560_091),
        None
    );
    assert_eq!(
        unix_epoch_difference(1_669_560_091 + 10, 1_669_560_091),
        None
    );
    assert_eq!(
        unix_epoch_difference(1_669_560_091 + 1, 1_669_560_091),
        None
    );
    assert_eq!(
        unix_epoch_difference(1_669_560_091 + 999_999, 1_669_560_091),
        None
    );
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

    for (param, expected) in assertions {
        assert_eq!(format_status(param), expected);
    }
}
