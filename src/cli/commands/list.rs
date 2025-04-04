// src/cli/commands/list.rs
use crate::cli::{command::Command, services::network::interfaces::InterfacesTrait};
use clap::Args;
use crate::cli::services::network::interfaces::InterfaceService;


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
        let service = InterfaceService::new();
        let interfaces = match &self.interfaces {
            Some(filter_names) => {
                println!("Filtrando pelas interfaces: {:?}", filter_names);
                service.filter_by_names(filter_names)
            },
            None => {
                println!("Listando todas as interfaces:");
                service.list_all()
            }
        };

        for interface in interfaces {
            println!("{}", service.display_interface(&interface, self.verbose));

        }
    }
}

