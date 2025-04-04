// src/network/interfaces.rs

// Não precisa mais do use pnet::datalink aqui (opcional)

pub trait InterfacesTrait {
    type NetworkInterface;
    
    fn new() -> Self;
    fn list_all(&self) -> Vec<Self::NetworkInterface>;
    fn filter_by_names(&self, names: &[String]) -> Vec<Self::NetworkInterface>;
    fn display_interface(&self, interface: &Self::NetworkInterface, verbose: bool) -> String;
}

pub struct InterfaceService;

impl InterfacesTrait for InterfaceService {
    type NetworkInterface = pnet::datalink::NetworkInterface;
    
    fn new() -> Self {
        Self
    }

    fn list_all(&self) -> Vec<Self::NetworkInterface> {
        pnet::datalink::interfaces()
    }

    fn filter_by_names(&self, names: &[String]) -> Vec<Self::NetworkInterface> {
        pnet::datalink::interfaces()
            .into_iter()
            .filter(|iface| names.contains(&iface.name))
            .collect()
    }

    fn display_interface(&self, interface: &Self::NetworkInterface, verbose: bool) -> String {
        if verbose {
            format!(
                "\nInterface: {}\n  MAC: {}\n  IPs: {:?}\n  Descrição: {}",
                interface.name,
                interface.mac.unwrap_or_default(),
                interface.ips,
                interface.description
            )
        } else {
            format!("- {}", interface.name)
        }
    }
}