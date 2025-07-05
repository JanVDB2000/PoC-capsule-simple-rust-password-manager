mod vault;
mod crypto;
mod commands;

use clap::Parser;
use commands::Cli;

fn main() {
    let cli = Cli::parse();
    commands::handle(cli);
}
