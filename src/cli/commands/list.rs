// src/cli/commands/list.rs
use crate::cli::command::Command;
use clap::Args;
use pnet::datalink;

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

        let all_interfaces = datalink::interfaces();
        
        match &self.interfaces {
            Some(filter_names) => {
                println!("Filtrando pelas interfaces: {:?}", filter_names);
                
                for interface in all_interfaces {
                    if filter_names.contains(&interface.name) {
                        self.print_interface_details(&interface);
                    }
                }
            },
            None => {
                println!("Listando todas as interfaces:");
                
                for interface in all_interfaces {
                    self.print_interface_details(&interface);
                }
            }
        }
    }
}

impl ListArgs {
    /// Método auxiliar para imprimir detalhes da interface
    fn print_interface_details(&self, interface: &datalink::NetworkInterface) {
        if self.verbose {
            println!("\nInterface: {}", interface.name);
            println!("  MAC: {}", interface.mac.unwrap_or_default());
            println!("  IPs: {:?}", interface.ips);
            println!("  Descrição: {}", interface.description);
        } else {
            println!("- {}", interface.name);
        }
    }
}