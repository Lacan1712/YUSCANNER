// src/cli/commands/list.rs
use crate::cli::command::Command;
use clap::Args;

#[derive(Args)]
pub struct ListArgs {
    /// Nome das interfaces para filtrar (opcional)
    #[clap(short, long)]
    pub interfaces: Option<Vec<String>>,
    
    /// Mostrar detalhes técnicos
    #[clap(short, long)]
    pub verbose: bool,
}

impl Command for ListArgs {
    fn execute(&self) {
        println!("Comando list selecionado");
        
        if self.verbose {
            println!("Modo verboso ativado");
        }
        
        match &self.interfaces {
            Some(names) => println!("Filtrando pelas interfaces: {:?}", names),
            None => println!("Listando todas as interfaces"),
        }
    }
}