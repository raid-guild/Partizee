use clap::Parser;

use std::collections::HashMap;
use std::path::PathBuf;

use crate::commands::account::{create_new_account, Account};
use crate::commands::compile::ProjectCompiler;
use crate::commands::deploy::{DeployConfigs, DeploymentWithAccount};
use crate::commands::new::NewProject;
use crate::utils::clap_cli::{AccountSubcommands, Cargo, Commands};
use crate::utils::menus::{
    compile_menu, create_new_account_menu, deploy_menu, select_account_menu,
};

pub fn partizee() -> Result<(), Box<dyn std::error::Error>> {
    let cargo_cli: Cargo = Cargo::parse();

    match cargo_cli {
        Cargo::Partizee(args) => {
            match args.commands {
                Commands::New {
                    name,
                    output_dir,
                    zero_knowledge,
                } => {
                    let new_project: NewProject = NewProject::new(name, output_dir);
                    // Pass zero_knowledge as needed
                    new_project.create_new_project()?;
                }
                Commands::Compile {
                    files_to_compile,
                    build_args,
                    additional_args,
                } => {
                    // create a new ProjectCompiler with the provided args
                    let compile_args: ProjectCompiler = ProjectCompiler {
                        project_root: None,
                        files: if files_to_compile.is_some() {
                            files_to_compile.map(|f| vec![f])
                        } else {
                            None
                        },
                        build_args: if build_args.is_some() {
                            build_args
                        } else {
                            None
                        },
                        additional_args: if additional_args.is_some() {
                            additional_args
                        } else {
                            None
                        },
                    };
                    // Use the interactive menu to get the compile args if none provided
                    let menu_args: ProjectCompiler = compile_menu(compile_args)?;

                    // create a new ProjectCompiler with the provided args
                    let project_compiler: ProjectCompiler = ProjectCompiler::new(menu_args);
                    // compile the contracts
                    project_compiler.compile_contracts()?;
                }
                Commands::Deploy {
                    interactive,
                    custom_net,
                    contract_names,
                    deploy_args,
                    account_path,
                } => {
                    let mut interactive: bool = interactive;
                    // if all args are empty open interactive menu
                    if interactive && custom_net.is_none() && contract_names.is_none()
                        && deploy_args.is_none()
                        && account_path.is_none()
                    {
                        interactive = true;
                    }
                    let mut deployer: DeploymentWithAccount;
                    // format deploy_args into a HashMap
                    let deployer_args: Option<HashMap<String, Vec<String>>> =
                        if deploy_args.is_some() {
                            let mut contract_map: HashMap<String, Vec<String>> = HashMap::new();
                            for entry in &deploy_args.unwrap() {
                                if let Some((name, args)) = entry.split_first() {
                                    contract_map.insert(name.clone(), args.to_vec());
                                }
                            }
                            Some(contract_map)
                        } else {
                            None
                        };

                    // format account_path into a PathBuf
                    let path_to_account: Option<PathBuf> = if account_path.is_some() {
                        Some(PathBuf::from(account_path.unwrap()))
                    } else {
                        None
                    };
                    // create a new DeployConfigs with the provided args
                    let config = DeployConfigs {
                        network: custom_net.clone(),
                        contract_names: contract_names.clone(),
                        deployer_args: deployer_args.clone(),
                        path_to_account: path_to_account.clone(),
                    };
                    // if interactive, get options from interactive menu and pass deployer_args as needed
                    if interactive {
                        let menu_args: DeployConfigs = deploy_menu(config)?;
                        if path_to_account.is_some() {
                            deployer =
                                DeploymentWithAccount::new(menu_args, path_to_account.clone());
                        } else {
                            deployer = DeploymentWithAccount::new(menu_args, None);
                        }
                        deployer.deploy_contracts();
                    } else {
                        let net: Option<String> = if custom_net.is_some() {
                            Some(custom_net.unwrap())
                        } else {
                            None
                        };
                        let names: Option<Vec<String>> = if contract_names.is_some() {
                            Some(contract_names.unwrap())
                        } else {
                            None
                        };

                        // create a new DeployProject with the provided args
                        let config = DeployConfigs {
                            network: net,
                            contract_names: names,
                            deployer_args: deployer_args,
                            path_to_account: None,
                        };
                        deployer = DeploymentWithAccount::new(config, None);
                        // deploy the contract
                        deployer.deploy_contracts();
                    }
                }
                Commands::Account { commands } => match commands {
                    AccountSubcommands::AccountCreate { shared_args } => {
                        if shared_args.interactive {
                            let account_args: Account =
                                create_new_account_menu().expect("Failed to create new account");
                            create_new_account(&account_args.network)?;
                        }
                    }
                    AccountSubcommands::AccountShow { shared_args } => {
                        if shared_args.interactive {
                            let accout_path: PathBuf =
                                select_account_menu().expect("Failed to select account");
                            let mut account: Account = Account::new(
                                Some(&accout_path),
                                shared_args.network.as_deref(),
                                None,
                                None,
                            )
                            .unwrap();
                            let account_output: String = account.show_account(
                                shared_args.network.as_deref(),
                                shared_args.address.as_deref(),
                            )?;
                            println!("{}", account_output);
                        } else {
                            if shared_args.network.is_some() && shared_args.address.is_some() {
                                let account: Account = Account::default();
                                let account_output: String = account.show_account(
                                    shared_args.network.as_deref(),
                                    shared_args.address.as_deref(),
                                )?;
                                println!("{}", account_output);
                            } else if shared_args.network.is_some() && shared_args.address.is_none()
                            {
                                let account: Account = Account::default();
                                let account_output: String = account.show_account(
                                    shared_args.network.as_deref(),
                                    shared_args.address.as_deref(),
                                )?;
                                println!("{}", account_output);
                            } else {
                                println!("No account found");
                            }
                        }
                    }
                    AccountSubcommands::AccountMintGas { shared_args } => {
                        if shared_args.interactive {
                            let account_path: PathBuf =
                                select_account_menu().expect("Failed to select account");
                            let account: Account = Account::new(
                                Some(&account_path),
                                shared_args.network.as_deref(),
                                None,
                                None,
                            )
                            .unwrap();
                            account.mint_gas().expect("Failed to mint gas");
                        } else {
                            let account: Account = Account::default();
                            account.mint_gas().expect("Failed to mint gas");
                        }
                    }
                },
            }
        }
    }

    Ok(())
}
