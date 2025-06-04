use clap::Parser;

use std::path::PathBuf;

use crate::commands::account::Account;
use crate::commands::compile::ProjectCompiler;
use crate::commands::deploy::DeployProject;
use crate::commands::new::NewProject;
use crate::utils::clap_cli::{AccountSharedArgs, AccountSubcommands, Cargo, Commands};
use crate::utils::menus::{new_account_menu, compile_menu, deploy_menu};

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
                    custom_net,
                    custom_path,
                    custom_root,
                    custom_deployer_args,
                } => {
                    let net: Option<String> = if custom_net.is_some() {
                        Some(custom_net.unwrap())
                    } else {
                        None
                    };
                    let path: Option<PathBuf> = if custom_path.is_some() {
                        Some(PathBuf::from(custom_path.as_ref().unwrap()))
                    } else {
                        None
                    };
                    let root: Option<PathBuf> = if custom_root.is_some() {
                        Some(PathBuf::from(custom_root.as_ref().unwrap()))
                    } else {
                        None
                    };
                    let deployer_args: Option<Vec<String>> = if custom_deployer_args.is_some() {
                        Some(custom_deployer_args.unwrap())
                    } else {
                        None
                    };

                    // create a new DeployProject with the provided args
                    let config = DeployProject {
                        network: net,
                        contract_path: path,
                        project_root: root,
                        deployer_args: deployer_args,
                        account_name: None,
                        account: None,
                    };

                    // get options from interactive menu and pass deployer_args as needed
                    let menu_args: DeployProject = deploy_menu(config)?;
                    // create a new DeployProject with the provided args
                    let mut deploy_project: DeployProject = DeployProject::new(menu_args);
                    // deploy the contract
                    deploy_project.deploy_contracts(None)?;
                }
                Commands::Account { commands } => match commands {
                    AccountSubcommands::AccountCreate { shared_args } => {
                        if shared_args.interactive {
                            let account_args: Account = new_account_menu().unwrap();
                            let mut account: Account = Account::new(
                                shared_args.name.as_deref(),
                                shared_args.network.as_deref(),
                                None,
                                None,
                            );
                            account.create_wallet(shared_args.network.as_deref());
                        } else {
                            let mut account: Account = Account::new(
                                shared_args.name.as_deref(),
                                shared_args.network.as_deref(),
                                None,
                                None,
                            );
                            account.create_wallet(shared_args.network.as_deref());
                        }
                    }
                    AccountSubcommands::AccountShow { shared_args } => {
                        if shared_args.interactive {
                            let mut account: Account = Account::new(
                                shared_args.name.as_deref(),
                                shared_args.network.as_deref(),
                                None,
                                None,
                            );;
                            account.show_account(shared_args.network.as_deref(), shared_args.address.as_deref());
                        } else {
                            let mut account: Account = Account::new(
                                shared_args.name.as_deref(),
                                shared_args.network.as_deref(),
                                None,
                                None,
                            );
                            account.show_account(shared_args.network.as_deref());
                        }
                    }
                    AccountSubcommands::AccountMintGas { shared_args } => {
                        if shared_args.interactive {
                            let account: Account = Account::new(
                                shared_args.name.as_deref(),
                                shared_args.network.as_deref(),
                                shared_args.path.as_deref(),
                                shared_args.public_key.as_deref(),
                                shared_args.address.as_deref(),
                                shared_args.account_index,
                            );
                            account.mint_gas();
                        } else {
                            let account: Account = Account::new(
                                shared_args.name.as_deref(),
                                shared_args.network.as_deref(),
                                shared_args.path.as_deref(),
                                shared_args.public_key.as_deref(),
                                shared_args.address.as_deref(),
                                shared_args.account_index,
                            );
                            account.mint_gas();
                        }
                    }
                },
            }
        }
    }

    Ok(())
}
