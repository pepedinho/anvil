use anvil::cli::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    println!("receive: {:#?}", cli.command);
    println!("Welcome to Anvil!");
}
