#![warn(clippy::all, clippy::pedantic)]

use std::{env, error::Error, path::PathBuf, time::Duration};

use duration_mins::DurationMins;

mod cancel;
mod daemon;
mod duration_mins;
mod parse;
mod start;
mod status;
mod unix_epoch;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<_>>();

    if args.len() == 1 {
        Err("Please provide a subcommand.")?;
    }

    let parsed_args =
        parse::start_arguments(&args.iter().map(<_>::as_ref).collect::<Vec<_>>()[2..]);

    let time_entry_path: PathBuf = [env::temp_dir(), PathBuf::from("time_entry")]
        .iter()
        .collect();

    let subcommand = args[1].to_lowercase();
    if subcommand == "start" {
        match &parsed_args.finished_script {
            Some(fs) => {
                start::start(
                    Duration::from_mins(parsed_args.duration_min),
                    &time_entry_path,
                    fs,
                );
            }
            None => Err("-f flag is required when using 'start'.".to_string())?,
        }
    } else if subcommand == "cancel" {
        if let Err(err) = cancel::cancel(&time_entry_path) {
            Err(err)?;
        };
    } else if subcommand == "status" {
        return status::status(&time_entry_path);
    } else {
        Err(format!("Could not handle subcommand {}.", subcommand))?;
    }

    Ok(())
}
