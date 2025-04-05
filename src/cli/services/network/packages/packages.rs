use pcap::{Device, Capture};

pub trait PackagesTrait {
    fn watch_interface(&self, name_interface: &str);
    fn new() -> Self;
}

pub struct PackagesService;

impl PackagesTrait for PackagesService {
    fn new() -> Self {
        Self
    }

    fn watch_interface(&self, name_interface: &str) {
        // Lista as interfaces e encontra a desejada
        let device = Device::list()
            .expect("Falha ao listar interfaces")
            .into_iter()
            .find(|iface| iface.name == name_interface)
            .expect("Interface não encontrada");

        println!("Abrindo interface: {}", device.name);

        // Cria captura em modo promíscuo
        let mut cap = Capture::from_device(device)
            .unwrap()
            .promisc(true)
            .snaplen(65535)
            .open()
            .expect("Não foi possível abrir a interface para captura");

        println!("Escutando pacotes na interface {}...", name_interface);

        while let Ok(packet) = cap.next() {
            println!("Pacote capturado: {} bytes", packet.data.len());

            // Aqui você pode fazer parsing manual se quiser
            // Exemplo de print do conteúdo (como hexdump básico):
            // println!("{:02x?}", &packet.data[..std::cmp::min(32, packet.data.len())]);
        }
    }
}
