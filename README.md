# Scanner CLI

[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Um utilitário CLI escrito em Rust que captura e analisa pacotes de rede utilizando a biblioteca `pcap`. Ideal para monitoramento e debugging de redes em ambientes Unix-like e Windows.

## Sobre o Projeto

Scanner é uma ferramenta CLI leve, eficiente e fácil de usar para:

- Listar interfaces de rede disponíveis.
- Capturar pacotes em tempo real com filtros avançados.
- Realizar parsing detalhado das camadas de rede, transporte e payloads dos pacotes capturados.

### Screenshot

![Product Screenshot](screenshot.png)

## Tecnologias Utilizadas

- [Rust](https://www.rust-lang.org/)
- [pcap](https://crates.io/crates/pcap)
- [etherparse](https://crates.io/crates/etherparse)
- [clap](https://crates.io/crates/clap)

## Começando

Estas instruções vão te ajudar a configurar o projeto localmente.

### Pré-requisitos

- Rust instalado ([rustup.rs](https://rustup.rs/))

#### Específico para Windows

No Windows é necessário instalar o [Npcap](https://npcap.com/#download), sucessor do WinPcap, para que a biblioteca pcap funcione corretamente.

### Instalação

Clone o repositório:

```sh
git clone https://github.com/seu_usuario/scanner.git
cd scanner
```

Compile o projeto:

```sh
cargo build --release
```

## Uso

```sh
# Listar interfaces disponíveis
scanner list

# Listar interfaces com detalhes
scanner list -v

# Capturar pacotes em uma interface específica
scanner watch -i eth0

# Capturar pacotes com filtro BPF
scanner watch -i eth0 -f "tcp port 80"
```

### Exemplos

Para capturar pacotes TCP na interface `eth0`:

```sh
scanner watch -i eth0 -f "tcp"
```

## Roadmap

- [x] Listagem básica de interfaces
- [x] Captura básica de pacotes
- [x] Suporte a filtros BPF
- [ ] Exportação para arquivos (ex: CSV, JSON)
- [ ] Interface gráfica opcional
- [ ] Suporte multi-threading

Consulte as [issues abertas](https://github.com/seu_usuario/scanner/issues) para ver a lista completa de recursos propostos e problemas conhecidos.

## Como Contribuir

Contribuições são bem-vindas e incentivadas!

1. Faça um fork do projeto
2. Crie sua branch de feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas alterações (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

## Licença

Distribuído sob a licença MIT. Veja `LICENSE` para mais informações.

## Contato

Rodrigo Lacan - [@RodrigoLacan](https://github.com/RodrigoLacan)

Link do projeto: [https://github.com/seu_usuario/scanner](https://github.com/seu_usuario/scanner)

## Agradecimentos

- [pcap Crate](https://crates.io/crates/pcap)
- [etherparse Crate](https://crates.io/crates/etherparse)
- [clap Crate](https://crates.io/crates/clap)
