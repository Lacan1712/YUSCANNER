// src/cli/args.rs
use clap::{Parser, Subcommand};
use crate::cli::commands;
use crate::cli::command::Command;

#[derive(Parser)]
#[clap(name = "scanner", version = "1.0", author = "@RodrigoLacan")]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Comando para listar interfaces de rede
    List(Box<commands::list::ListArgs>),
}

impl Commands {
    pub fn execute(&self) {
        match self {
            Commands::List(cmd) => cmd.execute(),
        }
    }
}