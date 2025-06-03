use cliclack::{confirm, input, intro, outro};
use crate::commands::new::ProjectConfig;
use crate::commands::compile::ProjectCompiler;
use crate::commands::deploy::DeployProject;
use crate::utils::utils::find_workspace_root;
use std::path::PathBuf;
use crate::commands::account::Account;

pub fn new_project_menu() -> Result<ProjectConfig, Box<dyn std::error::Error>> {
    intro("Partizee - Create a new Partisia Blockchain project")?;

    let name: String = input("What is your project name?")
        .placeholder("my-dapp")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Project name cannot be empty")
            } else {
                Ok(())
            }
        })
        .interact()?;

    let use_custom_dir = confirm("Would you like to specify a custom output directory? (if No: we'll create the project in the current directory with the name already specified)")
        .initial_value(false)
        .interact()?;

    let output_dir = if use_custom_dir {
        Some(
            input("Enter output directory path")
                .placeholder(&name)
                .interact()?,
        )
    } else {
        None
    };

    outro("Project configuration complete!")?;

    Ok(ProjectConfig { name, output_dir })
}

pub fn compile_menu(config: ProjectCompiler) -> Result<ProjectCompiler, Box<dyn std::error::Error>> {
    let mut build_args_vec: Vec<String> = Vec::new();
    let mut additional_args_vec: Vec<String> = Vec::new();
    let mut files_vec: Vec<String> = Vec::new();
    
    if config.files.is_none() {
        let use_file_menu = confirm("Would you like to specify which contracts to compile? (if No: we'll compile all contracts in the contracts directory with default settings)")
        .initial_value(false)
        .interact()?;
        if use_file_menu {
            loop {
                let file_to_compile: String = input("Enter the path to a Cargo.toml of the contract to compile")
                    .placeholder("contracts/counter/Cargo.toml")
                    .validate(|input: &String| {
                        let path = std::path::Path::new(input);
                        if !path.exists() {
                            Err("File does not exist")
                        } else if !path.is_file() {
                            Err("Path is not a file")
                        } else if path.file_name().map_or(true, |name| name != "Cargo.toml") {
                            Err("File must be named Cargo.toml")
                        } else {
                            // Optionally, check for [package] section
                            let content = std::fs::read_to_string(path);
                            if let Ok(content) = content {
                                if content.contains("[package]") {
                                    Ok(())
                                } else {
                                    Err("Cargo.toml does not contain a [package] section")
                                }
                            } else {
                                Err("Could not read file")
                            }
                        }
                    })
                    .interact()?;
                if !file_to_compile.trim().is_empty() {
                    files_vec.push(file_to_compile);
                }
                let another: bool = confirm("Enter another Contract?")
                    .initial_value(false)
                    .interact()?;
                    if !another {
                        break;
                    }
                }
            }
     }

    if config.build_args.is_none() {

        let add_build_args = confirm(
            "Would you like to specify Build arguments? \n 
        (if No: we'll compile all contracts in the contracts directory with default settings) \n
        Options: \n
        -r, --release                        Build artifacts in release mode, with optimizations \n
        -n, --no-abi                         Skip generating .abi file \n
        -q, --quiet                          No messages printed to stdout \n
        -w, --no-wasm-strip                  Do not remove custom sections from the WASM-file (will produce a much larger file). \n
        -z, --no-zk                          Only compile the public part of the contract. Skips compilation of ZK computation. \n
        --disable-git-fetch-with-cli     Uses cargo's built-in git library to fetch dependencies instead of the git executable \n
        --workspace                      Build all packages in the workspace \n
        --coverage                       Compile an instrumented binary for the smart contract. This enables generation of coverage files. \n
        -p, --package <PACKAGE>          Build only the specified packages \n
        -h, --help                       Print help \n
    ",
        )
        .initial_value(false)
        .interact()?;

    
        const AVAILABLE_FLAGS: &[&'static str; 17] = &["-r", "--release", "-n", "--no-abi", "-q", "--quiet", "-w", "--no-wasm-strip", "-z", "--no-zk", "--disable-git-fetch-with-cli", "--workspace", "--coverage", "-p", "--package", "-h", "--help"];

        if add_build_args {
            loop {
                let input_arg: String = input("Enter build argument ")
                    .placeholder("--help")
                    .validate(|input: &String| {
                        let input_arg = input.trim().to_string();
                        if AVAILABLE_FLAGS.iter().any(|flag| input_arg.contains(flag)) {
                            Ok(())
                        } else {
                            Err("Invalid build argument")
                        }
                    })
                    .interact()?;

                if !input_arg.trim().is_empty() {
                    build_args_vec.push(input_arg);
                }
                let another: bool = confirm("Enter another argument?")
                    .initial_value(false)
                    .interact()?;

                
                println!("build args: {:#?}", &build_args_vec);
                if !another {
                    break;
                }
            }
        }
    }

    if config.additional_args.is_none() {
        let add_additional_args = confirm(
            "Would you like to specify additional arguments? \n
     (if No: we'll compile all contracts in the contracts directory with default compiler arguments)",
        )
        .initial_value(false)
        .interact()?;

        if add_additional_args {
            loop {
                let additional_arg: String = input("Enter compiler argument:
                 (see https://partisiablockchain.gitlab.io/documentation/smart-contracts/smart-contract-tools-overview.html#command-line-tools for more info)")
                .placeholder("pbc cli arg")
                .interact()?;
                if !additional_arg.trim().is_empty() {
                    additional_args_vec.push(additional_arg);
                }
                let another: bool = confirm("Enter another argument? (y/n)")
                    .initial_value(false)
                    .interact()?;
                if !another {
                    break;
                }
            }
        }
    }

    let files: Option<Vec<String>> = if files_vec.len() > 0 {
        Some(files_vec)
    } else {
        None
    };

    let build_args: Option<Vec<String>> = if build_args_vec.len() > 0 {
        Some(build_args_vec)
    } else {
        None
    };

    let additional_args: Option<Vec<String>> = if additional_args_vec.len() > 0 {
        Some(additional_args_vec)
    } else {
        None
    };

    Ok(ProjectCompiler {
        project_root: None,
        files,
        build_args,
        additional_args,
    })
}

pub fn deploy_menu(config: DeployProject) -> Result<DeployProject, Box<dyn std::error::Error>> {
    let network: Option<String>;
    let path: Option<PathBuf>;
    let project_root: Option<PathBuf>;
    let mut deployer_args_vec: Vec<String> = Vec::new();
    if config.network.is_none() {
        let custom_network: String = input("Enter the network to deploy to eg. testnet, mainnet")
        .placeholder("testnet")
        .default_input("testnet")
        .interact()?;

        network = if !custom_network.trim().is_empty() {
            Some(custom_network)
        } else {
            None
        };
    } else {
        network = config.network;
    };

    if config.contract_path.is_none() {
        let use_custom_path = confirm("Would you like to specify a custom path to the contract to deploy? (if No: we'll use the default path)")
        .initial_value(false)
        .interact()?;
        if use_custom_path {
            let custom_path: String = input("Enter the path to the contract to deploy")
            .placeholder("contracts/counter/target/wasm32-unknown-unknown/release/counter.pbc")
            .validate(|input: &String| {

            if !input.trim().is_empty() {
                Err("File does not exist")
            } else if !PathBuf::from(input).is_file() {
                Err("Path is not a file")
            } else{
            Ok(())
            }
            })
            .interact()?; 
            path = Some(PathBuf::from(custom_path));
        } else {
            path = None;
        }
    } else {
        path = config.contract_path;
    };

    if config.deployer_args.is_none() {
        let add_deployer_args = confirm("Would you like to specify deployer arguments? (if No: we'll use the default arguments)")
        .initial_value(false)
        .interact()?;

        if add_deployer_args {
        loop {
            let deployer_arg: String = input("Enter deployer argument")
            .placeholder("pbc cli arg")
            .interact()?;
            if !deployer_arg.trim().is_empty() {
                deployer_args_vec.push(deployer_arg);
            }
            let another: bool = confirm("Enter another argument? (y/n)")
                .initial_value(false)
                .interact()?;
                if !another {
                    break;
                }
            }
        }
    }

    if config.project_root.is_none() {
        project_root = find_workspace_root();
    } else {
        project_root = config.project_root;
    };  

    let deployer_args: Option<Vec<String>> = if deployer_args_vec.len() > 0 {
        Some(deployer_args_vec)
    } else {
        None
    };

   
    Ok(DeployProject { 
        network: network,
        contract_path: path,
        project_root: project_root,
        deployer_args: deployer_args,
        account_name: None,
        account: None
     })
}

pub fn new_account_menu() -> Result<bool, Box<dyn std::error::Error>> {
   // ask if user wants to force create a new account
   let force_create:Result<_, std::io::Error> = confirm("Would you like to force create a new account? (yes will overwrite the existing account)")
   .initial_value(false)
   .interact();
   return Ok(force_create.unwrap());
}
