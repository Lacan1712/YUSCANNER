// src/main.rs
mod cli;

use clap::Parser;
use cli::args::Args;

fn main() {
    let args = Args::parse();
    args.command.execute();
}