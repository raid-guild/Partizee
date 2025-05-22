mod commands;

use std::{env, process};
use std::path::PathBuf;
use std::error::Error;

use commands::new::NewProject;
use commands::compile::ProjectCompiler;
use commands::deploy::DeployConfig;

const PROGRAM_NAME: &str = "partizee";

#[derive(Debug)]
enum Command {
    New(String, Option<String>),  // dapp_name, output_dir
    Compile(String), // project_name
    Deploy(String), // contract_name
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    println!("{:?}", args);
    args.next();
    
    let command = match args.next() {
        Some(cmd) => match cmd.as_str() {
            "new" => {
                let project_name = match args.next() {
                    Some(name) => name,
                    None => {
                        show_usage("Dapp name is required for 'new' command");
                        process::exit(1);
                    }
                };
                let output_dir = args.next();
                Command::New(project_name, output_dir)
            },
            "compile" => {
                let project_name = args.next().unwrap_or_default();
                Command::Compile(project_name)
            },
            "deploy" => {
                let contract_name = args.next().unwrap_or_default();
                Command::Deploy(contract_name)
            },
            _ => {
                show_usage(&format!("Unknown command: {}", cmd));
                process::exit(1);
            }
        },
        None => {
            show_usage("Command is required");
            process::exit(1);
        }
    };

    let result: Result<(), Box<dyn Error + 'static>> = match command {
        Command::New(dapp_name, output_dir) => {
            let new_project = NewProject::new(dapp_name, output_dir);
            new_project.create_new_project()?;
            Ok(())
        },
        Command::Compile(project_name) => {
            let project_compiler = ProjectCompiler::new();
            commands::compile::execute(project_compiler)
        },
        Command::Deploy(contract_name) => {
            let config = DeployConfig::new(PathBuf::from(contract_name));
            commands::deploy::execute(config)
        }
    };

    if let Err(e) = result {
        eprintln!("❌ Error: {}", e);
        process::exit(1);
    } else {
        Ok(())
    }
}

fn is_flag(arg: &str) -> bool {
    arg.starts_with("-")
}

fn show_usage(error: &str) {
    eprintln!("❌ Error: {}", error);
    eprintln!("\nUsage: {} <command> [arguments]", PROGRAM_NAME);
    eprintln!("\nCommands:");
    eprintln!("  new <dapp-name> [output-dir]    Create a new Partisia dapp");
    eprintln!("  compile                         Compile all contracts in the contracts directory");
    eprintln!("  deploy                         Deploy a contract (not yet implemented)");
    eprintln!("\nExamples:");
    eprintln!("  {} new my-dapp", PROGRAM_NAME);
    eprintln!("  {} compile", PROGRAM_NAME);
}
