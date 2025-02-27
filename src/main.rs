mod application;
mod cli;
mod domain;
mod infrastructure;

use clap::Parser;
use cli::commands::{Cli, run};

fn main() {
    let cli = Cli::parse();
    run(cli);
}
