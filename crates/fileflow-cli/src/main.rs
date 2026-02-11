use clap::{Parser, Subcommand};

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

            println!("Job: {} ({:?})", out.job.action_name, out.job.id);
            print_status(&out.job.status);

            if let Some(p) = &out.job.progress {
                println!(
                    "Progreso: {}/{} ({:?})",
                    p.current,
                    p.total,
                    p.message.as_deref().unwrap_or("")
                );
            }

            println!("\nLogs:");
            for l in out.logs {
                println!("[{:?}] {}", l.level, l.message);
            }
        }

        Commands::Actions { command } => match command {
            ActionsCommands::List => {
                println!("Acciones disponibles:");
                for a in actions::list_actions() {
                    println!("- {a}");
                }
            }
        },

        Commands::Ping => {
            println!("pong");
        },
        Commands::IntroRust => {
            intro_rust::intro_rust();
        },
    }
}


fn print_status(status: &JobStatus) {
    match status {
        JobStatus::Success => println!("Estado: SUCCESS"),
        JobStatus::Running => println!("Estado: RUNNING"),
        JobStatus::Pending => println!("Estado: PENDING"),
        JobStatus::Cancelled => println!("Estado: CANCELLED"),
        JobStatus::Failed(err) => println!("Estado: FAILED -> {err}"),
    }
}