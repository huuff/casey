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
                    .arg(arg!(-m --main "detect only the main case"))
                    .arg(arg!(-a --all "detect all cases with their frequencies"))
                    .group(
                        // TODO: Default of stdin
                        ArgGroup::new("input")
                                  .args(["file", "stdin"])
                                  .required(true) 
                        
                    )
                    .group(
                        // TODO: Default of main
                        ArgGroup::new("output")
                                  .args(["main", "all"])
                                  .required(true)
                    )
                    
                ) 
        .get_matches()
        ;
}
