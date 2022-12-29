use clap::{Parser, Subcommand, ArgGroup};

#[derive(Parser)]
#[command(author, version)]
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

        #[arg(short, long, group = "output")]
        report: bool,
    },
    Convert {
    },
}

