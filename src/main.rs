mod case;
mod detect;
mod matchers;
mod report;

use clap::{arg, command, Command, ArgGroup};

fn main() {
    // TODO: Can't I show the help instead of an error when no command is provided?
    // TODO: Correct cargo metadata
    let args = command!()
        .subcommand_required(true)
        .subcommand(
            Command::new("detect")
                    .about("detects used cases in an input")
                    .arg(arg!(-f --file [FILE] "from file"))
                    .arg(arg!(-i --stdin "from stdin"))
                    .arg(arg!(-m --main "detect only the main case"))
                    // TODO: Argument to decide the output (json, table, csv, etc.)
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

    match args.subcommand() {
       Some(("detect", sub_args)) => println!("Detect"),
       Some(("convert", sub_args)) => println!("Convert"),
       _ => unreachable!("Some subcommand must habe been chosen")
    }
}
