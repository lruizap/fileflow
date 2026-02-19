use clap::{Parser, Subcommand};
use std::path::PathBuf;

use fileflow_core::{Engine, JobStatus};
use fileflow_actions as actions;

mod intro_rust;

#[derive(Parser, Debug)]
#[command(name = "fileflow")]
#[command(about = "FileFlow - Automatizador local de tareas de archivos", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Ejecuta una acción por nombre
    Run {
        /// Nombre de la acción (ej: echo)
        action: String,
    },

    /// Operaciones relacionadas con acciones
    Actions {
        #[command(subcommand)]
        command: ActionsCommands,
    },

    /// Copia un archivo src -> dst
    Copy {
        src: PathBuf,
        dst: PathBuf,

        /// Sobrescribe si el destino existe
        #[arg(long)]
        overwrite: bool,
    },

    /// Comando de prueba para verificar que el CLI funciona
    Ping,

    // Comando de introducción a Rust, para mostrar ejemplos de sintaxis y conceptos básicos del lenguaje.
    // De esta forma obligo al comando a llamarse como yo indique.
    #[command(name = "IntroRust")]
    IntroRust,
}

#[derive(Subcommand, Debug)]
enum ActionsCommands {
    /// Lista las acciones disponibles
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { action } => {
            let Some(act) = actions::get_action(&action) else {
                eprintln!("Acción no encontrada: {action}");
                eprintln!("Usa: fileflow actions list");
                std::process::exit(1);
            };

            let engine = Engine::new();
            let out = engine.run_action(act.as_ref());
            print_output(out.job.status, out.logs);
        },

        Commands::Actions { command } => match command {
            ActionsCommands::List => {
                println!("Acciones disponibles:");
                for a in actions::list_actions() {
                    println!("- {a}");
                }
            }
        },

        Commands::Copy { src, dst, overwrite } => {
            let cfg = actions::CopyConfig { src, dst, overwrite };
            let act = actions::build_copy_action(cfg);

            let engine = Engine::new();
            let out = engine.run_action(act.as_ref());
            print_output(out.job.status, out.logs);
        },

        Commands::Ping => {
            println!("pong");
        },
        Commands::IntroRust => {
            intro_rust::intro_rust();
        },
    }
}



fn print_output(status: JobStatus, logs: Vec<fileflow_core::LogEntry>) {
    match &status {
        JobStatus::Success => println!("Estado: SUCCESS"),
        JobStatus::Failed(err) => println!("Estado: FAILED -> {err}"),
        JobStatus::Cancelled => println!("Estado: CANCELLED"),
        JobStatus::Running => println!("Estado: RUNNING"),
        JobStatus::Pending => println!("Estado: PENDING"),
    }

    println!("\nLogs:");
    for l in logs {
        println!("[{:?}] {}", l.level, l.message);
    }
}