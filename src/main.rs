mod case;
mod detect;
mod matchers;
mod report;
mod args;

use clap::Parser;
use args::Args;

fn main() {
    // TODO: Correct cargo metadata
    let args = Args::parse();

}
