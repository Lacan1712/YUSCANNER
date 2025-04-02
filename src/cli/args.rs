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
    /// Lista interfaces de rede
    List(Box<commands::list::ListArgs>),
    // Outros comandos...
}

// Implementação para executar qualquer comando
impl Commands {
    pub fn execute(&self) {
        match self {
            Commands::List(cmd) => cmd.execute(),
            // Outros comandos...
        }
    }
}