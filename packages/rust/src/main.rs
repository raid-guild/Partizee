mod commands;
pub mod utils;

use clap::Parser;
use std::path::PathBuf;

use utils::clap_cli::{Cargo, Commands};

use commands::new::NewProject;
use commands::compile::ProjectCompiler;
use commands::deploy::DeployProject;

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
                Commands::Compile { files_to_compile, build_args, additional_args } => {
                    // create a new ProjectCompiler with the provided args
                    let compile_args: ProjectCompiler = ProjectCompiler {
                        project_root: None,
                        files: if files_to_compile.is_some() { files_to_compile.map(|f| vec![f]) } else { None },
                        build_args: if build_args.is_some() { build_args } else { None },
                        additional_args: if additional_args.is_some() { additional_args } else { None },
                    };
                     // Use the interactive menu to get the compile args if none provided
                        let menu_args = utils::menus::compile_menu(compile_args)?;

                    // create a new ProjectCompiler with the provided args
                    let project_compiler = ProjectCompiler::new(menu_args);
                    // compile the contracts
                    project_compiler.compile_contracts()?;
                }
                Commands::Deploy { custom_net, custom_path, custom_root, custom_deployer_args } => {
                    let net = if custom_net.is_some() { Some(custom_net.unwrap()) } else { None };
                    let path = if custom_path.is_some() { Some(PathBuf::from(custom_path.unwrap())) } else { None };
                    let root = if custom_root.is_some() { Some(PathBuf::from(custom_root.unwrap())) } else { None };
                    let deployer_args = if custom_deployer_args.is_some() { Some(custom_deployer_args.unwrap()) } else { None };

                    // create a new DeployProject with the provided args    
                    let config = DeployProject {
                        network: net,
                        contract_path: path,
                        project_root: root,
                        deployer_args: deployer_args,
                    };

                    // get options from interactive menu and pass deployer_args as needed
                    let menu_args = utils::menus::deploy_menu(config)?;
                    // create a new DeployProject with the provided args
                    let deploy_project = DeployProject::new(menu_args);
                    // deploy the contract
                    deploy_project.deploy_contracts()?;
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
