mod case;
mod detect;
mod matchers;
mod report;
mod args;

use clap::Parser;
use args::{Args, Command};
use std::fs::File;
use std::error::Error;
use std::io::{self, BufReader, BufRead};
use report::{CaseReport, FrequencyCaseReport};

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Correct cargo metadata
    let args = Args::parse();

    match args.command {
        Command::Detect { file, stdin, main, report: print_report } => {
            let mut input_read: Box<dyn BufRead> = if let Some(file_name) = file {
                Box::new(BufReader::new(File::open(file_name)?))
            } else {
                Box::new(BufReader::new(io::stdin()))
            };

            let report = FrequencyCaseReport::from(&mut input_read)?;

            if !print_report {
                // Print only the main case
                if let Some(main_case) = report.main() {
                    println!("{}", main_case);
                } else {
                    println!("Unable to detect a case!");
                }
            }
            
        },
        Command::Convert {} => {

        }
    };

    Ok(())
}
