use clap::{Parser, Subcommand};

use fileflow_core::{Engine, JobStatus, LogEntry};
use fileflow_actions as actions;

#[derive(Parser, Debug)]
#[command(name = "fileflow")]
#[command(about = "FileFlow - Automatizador local de tareas de archivos", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Ejecuta una acción por nombre (usa -- para pasar args a la acción)
    ///
    /// Ej:
    ///   fileflow run echo
    ///   fileflow run copy -- --src a.txt --dst b.txt --overwrite
    Run {
        /// Nombre de la acción (ej: echo, copy)
        action: String,

        /// Argumentos de la acción (van después de --)
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Operaciones relacionadas con acciones
    Actions {
        #[command(subcommand)]
        command: ActionsCommands,
    },
}

#[derive(Subcommand, Debug)]
enum ActionsCommands {
    /// Lista las acciones disponibles
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { action, args } => {
            let engine = Engine::new();

            // args aquí ya contiene todo lo que venga después de `--`
            let act = match actions::build_action(&action, &args) {
                Ok(a) => a,
                Err(e) => {
                    eprintln!("Error: {e}");
                    eprintln!("Usa: fileflow actions list");
                    std::process::exit(1);
                }
            };

            let out = engine.run_action(act.as_ref());
            print_output(out.job.status, out.logs);
        }

        Commands::Actions { command } => match command {
            ActionsCommands::List => {
                println!("Acciones disponibles:");
                for (name, help) in actions::list_actions_help() {
                    println!("- {name}: {help}");
                }
            }
        },
    }
}

fn print_output(status: JobStatus, logs: Vec<LogEntry>) {
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
