#![warn(clippy::all, clippy::pedantic)]

use std::env;
mod daemon;
mod parse;
mod start;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() == 1 {
        eprintln!("Please provide a subcommand.");
        return;
    }

    let subcommand = args[1].to_lowercase();
    if subcommand == "start" {
        let start_args =
            parse::start_arguments(&args.iter().map(<_>::as_ref).collect::<Vec<_>>()[2..]);

        start::timer(start_args.duration_min);
    }
}
