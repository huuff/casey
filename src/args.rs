use clap::{Parser, Subcommand, ArgGroup, ValueEnum};
use crate::case::Case;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
#[command(group = ArgGroup::new("input_group").multiple(false))]
#[command(group = ArgGroup::new("output").multiple(false))]
pub enum Command {
    #[command(about = "Detect the cases used in an input")]
    Detect {
        #[arg(short, long, group = "input_group", help = "Detect cases from file")]
        file: Option<String>,

        #[arg(short, long, group = "input_group", help = "Detect cases from stdin")]
        stdin: bool,

        #[arg(
            short,
            long,
            group = "input", 
            help = "Detect from a single inline argument",
            conflicts_with = "report",
        )]
        inline: Option<String>,

        #[arg(short, long, group = "output_group", help = "Print only the most frequent case in input")]
        main: bool,

        #[arg(short, long, group = "output_group", help = "Print all used cases")]
        report: Option<Option<ReportType>>,
    },
    #[command(about = "Convert between case types")]
    Convert {
        #[arg(short, long, group = "input_group", help = "Converts cases from a file")]
        file: Option<String>,

        #[arg(short, long, group = "input_group", help = "Converts cases from stdin")]
        stdin: bool,

        #[arg(
            short,
            long,
            group = "input_group", 
            help = "Converts the case of a single inline argument",
        )]
        inline: Option<String>,

        #[arg(long, required=true)]
        from: Vec<Case>,

        #[arg(long, required=true)]
        to: Vec<Case>,

        #[arg(long, group = "output_group", help = "Print to stdout")]
        stdout: bool,

        #[arg(short, long, group = "output_group", help = "Print to file")]
        output: Option<String>,

    },
    #[command(about = "Generate completion scripts for a given shell")]
    Completions {
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum ReportType {
    #[value(help = "Times each case is used in input")]
    Frequency,
    #[value(help = "Proportion between 0 and 1 that each case is used in input")]
    Proportion,
    #[value(help = "Percentage of times each case is used in input")]
    Percentage,
}
