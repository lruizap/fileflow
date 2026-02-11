use clap::{Parser, Subcommand};

mod intro_rust;

#[derive(Parser, Debug)]
#[command(name = "fileflow", version, about = "FileFlow - Automatizador local de archivos (Rust)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Comando de prueba para verificar que el CLI funciona
    Ping,



    // Comando de introducción a Rust, para mostrar ejemplos de sintaxis y conceptos básicos del lenguaje.
    // De esta forma obligo al comando a llamarse como yo indique.
    #[command(name = "IntroRust")]
    IntroRust,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ping => {
            println!("pong");
        },
        Commands::IntroRust => {
            intro_rust::intro_rust();
        },
    }
}
