use clap::Parser;

#[derive(Parser)]
#[command(
    name = "postal_converter",
    version = "1.0",
    about = "Convert French addresses to ISO 20022 format"
)]
pub struct Cli {}

pub fn run(_cli: Cli) {
    println!("Hello, world!");
}