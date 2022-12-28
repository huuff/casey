use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(about, author)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Detect,
}
