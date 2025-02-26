mod domain;
mod infrastructure;
mod cli;

use clap::Parser;
use cli::commands::{Cli, run};

fn main() {
    let cli = Cli::parse();
    run(cli);
}
