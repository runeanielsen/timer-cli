#![warn(clippy::all, clippy::pedantic)]

use crate::config::Config;
use std::env;

mod cancel;
mod config;
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

    // We allow deprecated since this application is only meant to work on Unix systems,
    // and most libraries dealing with finding home_dir path
    // uses this function underneath anyway.
    #[allow(deprecated)]
    let config = Config::new(env::home_dir().unwrap());

    let subcommand = args[1].to_lowercase();
    if subcommand == "start" {
        let start_args =
            parse::start_arguments(&args.iter().map(<_>::as_ref).collect::<Vec<_>>()[2..]);

        start::start(start_args.duration_min, &config);
    } else if subcommand == "cancel" {
        cancel::cancel(&config);
    } else if subcommand == "status" {
        status::status(&config);
    } else {
        eprintln!("Could not handle subcommand: '{}'.", subcommand);
    }
}
