#![warn(clippy::all, clippy::pedantic)]

use std::{env, path::PathBuf};

mod cancel;
mod daemon;
mod parse;
mod start;
mod status;
mod unix_epoch;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() == 1 {
        eprintln!("Please provide a subcommand.");
        return;
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
                start::start(parsed_args.duration_min, &time_entry_path, &fs);
            }
            None => {
                eprintln!("-f parameter is required when using 'start' subcommand.");
            }
        }
    } else if subcommand == "cancel" {
        cancel::cancel(&time_entry_path);
    } else if subcommand == "status" {
        status::status(&time_entry_path);
    } else {
        eprintln!("Could not handle subcommand: '{}'.", subcommand);
    }
}
