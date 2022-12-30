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
use std::io::Cursor;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command {
        Command::Detect { file, stdin: _, mut inline, main: _, report: print_report } => {
            let mut input_read: Box<dyn BufRead> = if let Some(file_name) = file {
                Box::new(BufReader::new(File::open(file_name)?))
            } else if let Some(token) = inline.take() {
                Box::new(BufReader::new(Cursor::new(token.into_bytes())))
            } else {
                Box::new(BufReader::new(io::stdin()))
            };

            let report = FrequencyCaseReport::from(&mut input_read)?;

            if let Some(print_report) = print_report {
                // Print the full report
                let report_type = print_report.unwrap_or(ReportType::Percentage);

                if let Some(report) = report {
                    let report: Box<dyn Display> = match report_type {
                        ReportType::Frequency => Box::new(report),
                        ReportType::Proportion => Box::new(report.proportions()),
                        ReportType::Percentage => Box::new(report.proportions().as_percentages()?),
                    };

                    println!("{}", report);
                } else {
                    eprintln!("Unable to detect a case!");
                    std::process::exit(1);
                }

            } else {
                // Print only the main case
                if let Some(report) = report {
                    println!("{}", report.main());
                } else {
                    eprintln!("Unable to detect a case!");
                    std::process::exit(1);
                }
            }
        },
        Command::Convert { file, stdin: _, mut inline, } => {
            let mut input_read: Box<dyn BufRead> = if let Some(file_name) = file {
                Box::new(BufReader::new(File::open(file_name)?))
            } else if let Some(token) = inline.take() {
                Box::new(BufReader::new(Cursor::new(token.into_bytes())))
            } else {
                Box::new(BufReader::new(io::stdin()))
            };
        }
    };

    Ok(())
}
