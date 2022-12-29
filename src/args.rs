use clap::{Parser, Subcommand, ArgGroup};

#[derive(Parser)]
#[command(author, version)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
#[command(group = clap::ArgGroup::new("input").multiple(false))]
pub enum Command {
    Detect {
        #[arg(short, long, group = "input")]
        file: Option<String>,

        #[arg(short, long, group = "input")]
        stdin: bool,

    },
    Convert {
    },
}

