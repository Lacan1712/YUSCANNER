// src/main.rs
mod cli;
use clap::Parser;
use cli::args::Args;
use std::path::Path;


fn check_npcap_installed() -> bool {
    Path::new(r"C:\Windows\System32\Npcap").exists()
}

fn main() {
    
    if cfg!(target_os = "windows") && !check_npcap_installed() {
        eprintln!("⚠️ O Npcap não está instalado no seu sistema.");
        eprintln!("Por favor, instale-o em: https://npcap.com/#download");
        std::process::exit(1);
    }

    let args = Args::parse();
    args.command.execute();
}