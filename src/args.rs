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
// TODO: Descriptions for arguments
pub enum Command {
    Detect {
        #[arg(short, long, group = "input")]
        file: Option<String>,

        #[arg(short, long, group = "input")]
        stdin: bool,

        #[arg(short, long, group = "output")]
        main: bool,

        #[arg(short, long, group = "output", )]
        report: Option<Option<ReportType>>,
    },
    Convert {
    },
}


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum ReportType {
    Frequency,
    Proportion,
    Percentage,
}
