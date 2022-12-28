mod case;
mod detect;
mod matchers;
mod report;

use clap::{arg, command, Command, ArgGroup};

fn main() {
    let args = command!()
        .subcommand_required(true)
        .subcommand(
            Command::new("detect")
                    .about("detects used cases in an input")
                    .arg(arg!(-f --file [FILE] "from file"))
                    .arg(arg!(-i --stdin "from stdin"))
                    .group(
                        ArgGroup::new("input")
                                  .args(["file", "stdin"])
                                  .required(true) 
                    )
                ) 
        .get_matches()
        ;
}
