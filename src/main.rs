mod case;
mod detect;
mod matchers;
mod report;
mod args;
mod convert;

use clap::Parser;
use args::{Args, Command, ReportType};
use std::fs::File;
use std::error::Error;
use std::io::{self, BufReader, BufRead};
use report::FrequencyCaseReport;
use std::fmt::Display;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command {
        Command::Detect { file, stdin: _, main: _, report: print_report } => {
            let mut input_read: Box<dyn BufRead> = if let Some(file_name) = file {
                Box::new(BufReader::new(File::open(file_name)?))
            } else {
                Box::new(BufReader::new(io::stdin()))
            };

            let report = FrequencyCaseReport::from(&mut input_read)?;

            if let Some(print_report) = print_report {
                // Print the full report
                let report_type = print_report.unwrap_or(ReportType::Percentage);

                let report: Box<dyn Display> = match report_type {
                    ReportType::Frequency => Box::new(report),
                    ReportType::Proportion => Box::new(report.proportions()),
                    ReportType::Percentage => Box::new(report.proportions().as_percentages()?),
                };

                println!("{}", report);

            } else {
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
