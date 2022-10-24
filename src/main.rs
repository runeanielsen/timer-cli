#![warn(clippy::all, clippy::pedantic)]

use std::env;

mod config;
mod daemon;
mod parse;
mod start;
mod time_entry;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() == 1 {
        eprintln!("Please provide a subcommand.");
        return;
    }

    let config_dir_path = config::dir_path();

    let subcommand = args[1].to_lowercase();
    if subcommand == "start" {
        let start_args =
            parse::start_arguments(&args.iter().map(<_>::as_ref).collect::<Vec<_>>()[2..]);

        start::timer(start_args.duration_min, &config_dir_path);
    }
}
