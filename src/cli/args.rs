// src/args.rs
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "scanner", version = "1.0", author = "@RodrigoLacan")]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Lista interfaces de rede
    List {
        /// Nome das interfaces para filtrar (opcional)
        #[clap(short, long)]
        interfaces: Option<Vec<String>>,
        
        /// Mostrar detalhes técnicos
        #[clap(short, long)]
        verbose: bool,
    },
}