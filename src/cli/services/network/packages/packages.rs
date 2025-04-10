use pcap::{Device, Capture};
use etherparse::PacketHeaders;

pub trait PackagesTrait {
    fn watch_interface(&self, name_interface: &str);
    fn watch_interface_by_filter(&self, name_interface: &str, filter: &Option<String>);
    fn new() -> Self;
}

pub struct PackagesService;

impl PackagesService {
    fn search_device_name(&self, name_interface: &str) -> Device {
        Device::list()
            .expect("Falha ao listar interfaces")
            .into_iter()
            .find(|iface| iface.name == name_interface)
            .expect("Interface não encontrada")
    }
}

impl PackagesTrait for PackagesService {
    fn new() -> Self {
        Self
    }

    fn watch_interface(&self, name_interface: &str) {
        let device = self.search_device_name(name_interface);
        let mut cap = Capture::from_device(device)
            .unwrap()
            .promisc(true)
            .snaplen(65535)
            .open()
            .expect("Não foi possível abrir a interface para captura");

        println!("Escutando pacotes na interface '{}'...\n", name_interface);

        while let Ok(packet) = cap.next() {
            println!("=======================================");
            println!("📦 Pacote capturado: {} bytes", packet.data.len());
            println!("=======================================\n");
        }
    }

    fn watch_interface_by_filter(&self, name_interface: &str, filter: &Option<String>) {
        let device = self.search_device_name(name_interface);
        let mut cap = Capture::from_device(device)
            .unwrap()
            .promisc(true)
            .snaplen(65535)
            .open()
            .expect("Não foi possível abrir a interface para captura");

        println!("Escutando pacotes na interface '{}'...\n", name_interface);

        let filtro_str = filter.as_deref().unwrap_or("");
        match cap.filter(filtro_str) {
            Ok(_) => println!("✅ Filtro aplicado: '{}'", filtro_str),
            Err(e) => eprintln!("❌ Erro ao aplicar filtro: {:?}", e),
        }

        while let Ok(packet) = cap.next() {
            println!("\n---------------------------------------");
            println!("📦 Pacote capturado: {} bytes", packet.data.len());

            if let Ok(headers) = PacketHeaders::from_ethernet_slice(packet.data) {
                if let Some(link) = headers.link {
                    match link {
                        etherparse::LinkHeader::Ethernet2(link) => {
                            println!("🔗 Link Layer:");
                            println!("   MAC Source:      {:?}", link.source);
                            println!("   MAC Destination: {:?}", link.destination);
                        }
                        etherparse::LinkHeader::LinuxSll(link) => {
                            println!("🔗 Link Layer (LinuxSLL):");
                            println!("   Sender: {:?}", link.sender_address);
                        }
                    }
                }

                if let Some(net) = headers.net {
                    println!("🌐 Network Layer:");
                    match net {
                        etherparse::NetHeaders::Ipv4(h, _) => {
                            println!("   IPv4 Source:      {:?}", h.source);
                            println!("   IPv4 Destination: {:?}", h.destination);
                        }
                        etherparse::NetHeaders::Ipv6(h, _) => {
                            println!("   IPv6 Source:      {:?}", h.source);
                            println!("   IPv6 Destination: {:?}", h.destination);
                        }
                        _ => println!("   ⚠️ Outro tipo de header de rede"),
                    }
                }

                if let Some(transport) = headers.transport {
                    println!("🚚 Transport Layer:");
                
                    match transport {
                        etherparse::TransportHeader::Tcp(tcp) => {
                            println!("   🔹 TCP");
                            println!("     ├─ Source Port:      {}", tcp.source_port);
                            println!("     ├─ Destination Port: {}", tcp.destination_port);
                            println!("     ├─ Seq Number:       {}", tcp.sequence_number);
                            println!("     ├─ Ack Number:       {}", tcp.acknowledgment_number);
                            println!("     ├─ Flags:");
                            println!("     │   SYN: {}  ACK: {}  FIN: {}  RST: {}  PSH: {}  URG: {}", 
                                tcp.syn, tcp.ack, tcp.fin, tcp.rst, tcp.psh, tcp.urg);
                            println!("     ├─ Window Size:      {}", tcp.window_size);
                            println!("     ├─ Checksum:         {}", tcp.checksum);
                            println!("     └─ Options:");
                            for opt in tcp.options.iter() {
                                println!("         • {:?}", opt);
                            }
                        }
                        etherparse::TransportHeader::Udp(udp) => {
                            println!("   🔹 UDP");
                            println!("     ├─ Source Port:      {}", udp.source_port);
                            println!("     ├─ Destination Port: {}", udp.destination_port);
                            println!("     ├─ Length:           {}", udp.length);
                            println!("     └─ Checksum:         {}", udp.checksum);
                        }
                        etherparse::TransportHeader::Icmpv4(icmp) => {
                            println!("   🔹 ICMPv4");
                            println!("     └─ Header: {:?}", icmp);
                        }
                        etherparse::TransportHeader::Icmpv6(icmp) => {
                            println!("   🔹 ICMPv6");
                            println!("     └─ Header: {:?}", icmp);
                        }
                    }
                }
                

                println!("📄 Payload:");
                match headers.payload {
                    etherparse::PayloadSlice::Empty => {
                        println!("   🔸 Sem payload útil.");
                    }
                    etherparse::PayloadSlice::Tcp(data) => {
                        println!("   🔸 TCP Payload: {} bytes", data.len());
                        if let Ok(texto) = std::str::from_utf8(data) {
                            println!("   🔸 Conteúdo textual (ex. HTTP):\n{}", texto);
                        } else {
                            println!("   ⚠️ Payload TCP não é texto válido UTF-8");
                        }
                    }
                    etherparse::PayloadSlice::Udp(data) => {
                        println!("   🔸 UDP Payload: {} bytes", data.len());
                        println!("   🔸 Hex: {:02X?}", data);
                    }
                    etherparse::PayloadSlice::Icmpv4(data) => {
                        println!("   🔸 ICMPv4 Payload: {:?}", data);
                    }
                    etherparse::PayloadSlice::Icmpv6(data) => {
                        println!("   🔸 ICMPv6 Payload: {:?}", data);
                    }
                    etherparse::PayloadSlice::Ip(inner) => {
                        println!("   🔸 IP Interno: {:?}", inner.payload);
                    }
                    etherparse::PayloadSlice::Ether(inner) => {
                        println!("   🔸 Ethernet Interno: {:?}", inner.payload);
                    }
                }
            } else {
                println!("⚠️ Não foi possível fazer o parse do pacote.");
            }

            println!("---------------------------------------\n");
        }
    }
}
