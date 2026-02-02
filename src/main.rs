use clap::{Parser, Subcommand};
use shellbridge::core::os_detector::{self, Platform};
use shellbridge::translation::engine::TranslationEngine;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "shellbridge")]
#[command(author = "Nikhil Kottoli")]
#[command(version = "1.0")]
#[command(about = "Translates shell commands between operating systems", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

use shellbridge::core::executor;

#[derive(Subcommand)]
enum Commands {
    /// Translate a command
    Translate {
        /// The command to translate
        cmd: String,
        
        /// Target OS (linux, macos, windows). Defaults to current OS.
        #[arg(short, long)]
        target: Option<String>,

        /// Execute the translated command directly
        #[arg(short, long)]
        execute: bool,
    },
    /// Explain a command
    Explain {
        /// The command to explain
        cmd: String,
    },
}

fn main() {
    let cli = Cli::parse();
    
    // Detect OS
    let current_os = os_detector::detect_os();
    
    // Load Translation Engine
    // For now we assume data/commands.json is in the current directory
    let db_path = PathBuf::from("data/commands.json");
    let engine = match TranslationEngine::new(db_path) {
        Ok(e) => e,
        Err(err) => {
            eprintln!("Error loading command database: {}", err);
            return;
        }
    };

    match &cli.command {
        Some(Commands::Translate { cmd, target, execute }) => {
            let target_os = if let Some(t) = target {
                match t.to_lowercase().as_str() {
                    "linux" => Platform::Linux,
                    "macos" | "darwin" => Platform::MacOS,
                    "windows" | "win32" => Platform::Windows,
                    "wsl" => Platform::WSL,
                    "docker" => Platform::Docker,
                    "gitbash" => Platform::GitBash,
                    _ => {
                        eprintln!("Unknown target platform: {}", t);
                        return;
                    }
                }
            } else {
                current_os.clone()
            };

            eprintln!("Translating '{}' to {:?}...", cmd, target_os);
            
            let translated = engine.translate(cmd, &target_os);
            
            match translated {
                Some(t_cmd) => {
                    println!("{}", t_cmd);
                    if *execute {
                        if let Err(e) = executor::execute_command(&t_cmd) {
                            eprintln!("Execution error: {}", e);
                        }
                    }
                },
                None => println!("No direct translation found."),
            }
        }
        Some(Commands::Explain { cmd }) => {
            println!("Explaining '{}'...", cmd);
            match engine.explain(cmd) {
                Some(explanation) => println!("\n{}", explanation),
                None => println!("Could not explain command."),
            }
        }
        None => {
            println!("No command provided. Use --help for usage.");
        }
    }
}
