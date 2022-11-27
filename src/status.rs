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
        // the end time is greater than the current time.
        if end_unix_epoch > now_unix_epoch {
            end_unix_epoch - now_unix_epoch
        } else {
            0
        }
    } else {
        0
    };

    println!("{}", format_status(unix_epoch_left));

    Ok(())
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

    for (param, expected) in assertions {
        assert_eq!(format_status(param), expected);
    }
}
