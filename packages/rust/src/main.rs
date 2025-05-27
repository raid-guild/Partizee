mod commands;
pub mod utils;

use clap::Parser;
use std::path::PathBuf;

use utils::clap_cli::{Cargo, Commands};
use utils::menus::CompileArgs;
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
    let cargo_cli: Cargo = Cargo::parse();

    match cargo_cli {
        Cargo::Partizee(args) => {
            match args.commands {
                Commands::New { name, output_dir, zero_knowledge } => {
                    let new_project = NewProject::new(name, output_dir);
                    // Pass zero_knowledge as needed
                    new_project.create_new_project()?;
                }
                Commands::Compile { file, build_args, additional_args } => {
                    // If no CLI args are provided, open the cliclack menu
                    let compile_args = if file.is_none() && build_args.is_empty() && additional_args.is_empty() {
                        // Use the interactive menu
                        let menu_args = utils::menus::compile_menu()?;
                        CompileArgs {
                            files: menu_args.files,
                            build_args: menu_args.build_args,
                            additional_args: menu_args.additional_args,
                        }
                    } else {
                        // Use CLI args
                        CompileArgs {
                            files: file.map(|f| vec![f]),
                            build_args: if !build_args.is_empty() { Some(build_args) } else { None },
                            additional_args: if !additional_args.is_empty() { Some(additional_args) } else { None },
                        }
                    };
                    let project_compiler = ProjectCompiler::new(compile_args);
                    project_compiler.compile_contracts()?;
                }
                Commands::Deploy { net, deployer_args } => {
                    let config = DeployConfig::new(PathBuf::from(net.unwrap_or_else(|| "testnet".to_string())));
                    // Pass deployer_args as needed
                    commands::deploy::execute(config)?;
                }
            }
        }
    }

    Ok(())
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
