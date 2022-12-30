use clap::{Parser, Subcommand, ArgGroup, ValueEnum};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
#[command(group = ArgGroup::new("input").multiple(false))]
#[command(group = ArgGroup::new("output").multiple(false))]
pub enum Command {
    #[command(about = "Detect the cases used in an input")]
    Detect {
        #[arg(short, long, group = "input", help = "Detect cases from file")]
        file: Option<String>,

        #[arg(short, long, group = "input", help = "Detect cases from stdin")]
        stdin: bool,

        #[arg(
            short,
            long,
            group = "input", 
            help = "Detect from a single inline argument",
            conflicts_with = "report",
        )]
        inline: Option<String>,

        #[arg(short, long, group = "output", help = "Print only the most frequent case in input")]
        main: bool,

        #[arg(short, long, group = "output", help = "Print all used cases")]
        report: Option<Option<ReportType>>,
    },
    #[command(about = "Convert between case types")]
    Convert {
    },
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
