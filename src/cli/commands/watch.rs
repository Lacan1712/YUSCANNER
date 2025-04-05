use clap::Args;
use crate::cli::{command::Command, services::network::packages::PackagesTrait};
use crate::cli::services::network::packages::PackagesService;


#[derive(Args)]
pub struct WatchArgs {
    /// Filtrar pacotes por interface
    #[clap(short, long)]
    pub interface: String,
    
    /// Mostrar detalhes técnicos
    #[clap(short, long)]
    pub verbose: bool,
}


impl Command for WatchArgs {

    fn execute(&self) {
        let packages_services = PackagesService::new();

        packages_services.watch_interface(&self.interface);
    }
}

