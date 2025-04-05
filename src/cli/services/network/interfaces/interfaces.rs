use pcap::Device;

pub trait InterfacesTrait {
    type NetworkInterface;

    fn new() -> Self;
    fn list_all(&self) -> Vec<Self::NetworkInterface>;
    fn filter_by_names(&self, names: &[String]) -> Vec<Self::NetworkInterface>;
    fn display_interface(&self, interface: &Self::NetworkInterface, verbose: bool) -> String;
}

pub struct InterfaceService;

impl InterfacesTrait for InterfaceService {
    type NetworkInterface = Device;

    fn new() -> Self {
        Self
    }

    fn list_all(&self) -> Vec<Self::NetworkInterface> {
        Device::list().expect("Não foi possível listar interfaces")
    }

    fn filter_by_names(&self, names: &[String]) -> Vec<Self::NetworkInterface> {
        Device::list()
            .expect("Não foi possível listar interfaces")
            .into_iter()
            .filter(|dev| names.contains(&dev.name))
            .collect()
    }

    fn display_interface(&self, interface: &Self::NetworkInterface, verbose: bool) -> String {
        if verbose {
            format!(
                "\nInterface: {}\n  Descrição: {:?}",
                interface.name,
                interface.desc
            )
        } else {
            format!("- {}", interface.name)
        }
    }
}
