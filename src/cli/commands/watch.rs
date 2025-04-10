use clap::Args;
use crate::cli::{command::Command, services::network::packages::PackagesTrait};
use crate::cli::services::network::packages::PackagesService;


#[derive(Args)]
pub struct WatchArgs {
    /// Filtrar pacotes por interface
    #[clap(short, long)]
    pub interface: String,
    
    
    /// Filtro do tipo BPM como "tcp port 80"
    #[clap(short, long)]
    pub filter: Option<String>,
}


impl Command for WatchArgs {

    fn execute(&self) {
        let packages_services = PackagesService::new();

        match self.filter.as_deref() {
            Some(f) if f.contains("tcp") => {
                println!("Filtro TCP: {}", f);
                packages_services.watch_interface_by_filter(&self.interface, &self.filter)
            },
            Some(f) => {
                println!("Filtro: {}", f)
            },
            None => {
                packages_services.watch_interface(&self.interface)
            }
        }

    }
}

