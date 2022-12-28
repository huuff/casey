mod case;
mod detect;
mod matchers;
mod report;
mod args;

use clap::Parser;
use args::Args;

fn main() {
    let args = Args::parse();
}
