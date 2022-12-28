mod case;
mod detect;
mod matchers;
mod report;

use clap::{command, Command};

fn main() {
    let args = command!()
        .subcommand_required(true)
        .subcommand(
            Command::new("detect")
            .about("detects used cases in an input")
        )
        .get_matches()
        ;
}
