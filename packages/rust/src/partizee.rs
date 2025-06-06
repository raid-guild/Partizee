use clap::Parser;

use std::collections::HashMap;
use std::path::PathBuf;

use crate::commands::account::{pbc_create_new_account, Account, AccountConfig};
use crate::commands::compile::ProjectCompiler;
use crate::commands::deploy::{DeployConfigs, Deployer, DeploymentWithAccount};
use crate::commands::new::{NewProject, ProjectConfig};
use crate::utils::clap_cli::{AccountSubcommands, Cargo, Commands};
use crate::utils::fs_nav::get_all_contract_names;
use crate::utils::menus::{
    compile_menu, create_new_account_menu, deploy_menu, new_project_menu, select_account_menu,
};

#[allow(unused_variables, unused_assignments)]
pub fn partizee() -> Result<(), Box<dyn std::error::Error>> {
    let cargo_cli: Cargo = Cargo::parse();

    match cargo_cli {
        Cargo::Partizee(args) => {
            match args.commands {
                Commands::New {
                    interactive,
                    name,
                    output_dir,
                    zero_knowledge, // for future use
                } => {
                    let new_project: NewProject;
                    if interactive {
                        let menu_args: ProjectConfig = new_project_menu(name, output_dir)?;
                        new_project = NewProject::new(menu_args);
                    } else {
                        new_project = NewProject::new(ProjectConfig {
                            name: name.expect("must provide name for new project"),
                            output_dir: output_dir,
                        });
                    }
                    // Pass zero_knowledge as needed
                    new_project.create_new_project()?;
                }
                Commands::Compile {
                    interactive,
                    files_to_compile,
                    build_args,
                    additional_args,
                } => {
                    // create a new ProjectCompiler with the provided args
                    let compile_args: ProjectCompiler = ProjectCompiler {
                        files: files_to_compile,
                        build_args: build_args,
                        additional_args: additional_args,
                    };

                    let project_compiler: ProjectCompiler;
                    if interactive {
                        let menu_args: ProjectCompiler = compile_menu(compile_args)?;
                        project_compiler = ProjectCompiler::new(menu_args);
                    } else {
                        project_compiler = ProjectCompiler::new(compile_args);
                    }

                    project_compiler.compile_contracts()?;
                }
                Commands::Deploy {
                    interactive,
                    custom_net,
                    contract_names,
                    deploy_args,
                    account_path,
                } => {
                    let mut use_interactive: bool = interactive;
                    // if all args are empty open interactive menu
                    if !interactive
                        && custom_net.is_none()
                        && contract_names.is_none()
                        && deploy_args.is_none()
                        && account_path.is_none()
                    {
                        use_interactive = true;
                    }
                    let mut deployer: DeploymentWithAccount;

                    // format account_path into a PathBuf
                    let path_to_account: Option<PathBuf> =
                        account_path.clone().map(|path| PathBuf::from(path));

                    let mut contracts_to_deploy: Option<Vec<String>> = None;
                    // get list of all contract names

                    if contract_names.is_none() {
                        contracts_to_deploy = Some(get_all_contract_names()?);
                    } else {
                        contracts_to_deploy = contract_names;
                    }
                    let mut deployer_args_hashmap: Option<HashMap<String, Vec<String>>> = None;
                    if contracts_to_deploy.is_some() {
                        deployer_args_hashmap = Some(parse_deploy_args(
                            deploy_args,
                            contracts_to_deploy.clone().unwrap(),
                        )?);
                    } else {
                        deployer_args_hashmap = None;
                    }
                    // create a new DeployConfigs with the provided args
                    let config = DeployConfigs {
                        network: custom_net,
                        contract_names: contracts_to_deploy,
                        deployer_args: deployer_args_hashmap,
                        path_to_account: path_to_account,
                    };
                    // if interactive, get options from interactive menu and pass deployer_args as needed
                    if interactive {
                        let menu_args: DeployConfigs = deploy_menu(config)?;
                        let deployer_args: Deployer = Deployer {
                            network: menu_args.network.unwrap_or("".to_string()),
                            contract_names: menu_args.contract_names.unwrap_or(Vec::new()),
                            deployer_args: menu_args.deployer_args.unwrap_or(HashMap::new()),
                            path_to_account: menu_args.path_to_account.unwrap_or(PathBuf::from("")),
                        };

                        deployer = DeploymentWithAccount::new(deployer_args);
                    } else {
                        let deployer_args: Deployer = Deployer {
                            network: config.network.unwrap_or("".to_string()),
                            contract_names: config.contract_names.unwrap_or(Vec::new()),
                            deployer_args: config.deployer_args.unwrap_or(HashMap::new()),
                            path_to_account: config.path_to_account.unwrap_or(PathBuf::from("")),
                        };
                        deployer = DeploymentWithAccount::new(deployer_args);
                    }

                    let result = deployer.deploy_contracts();
                    if result.is_ok() {
                        println!("Contracts deployed successfully");
                    } else {
                        println!("Contracts deployment failed");
                    }
                }
                Commands::Account { commands } => match commands {
                    AccountSubcommands::AccountCreate { shared_args } => {
                        if shared_args.interactive {
                            let account_args: Account =
                                create_new_account_menu().expect("Failed to create new account");
                            pbc_create_new_account(&account_args.network)?;
                        }
                    }
                    AccountSubcommands::AccountShow { shared_args } => {
                        if shared_args.interactive {
                            let accout_path: PathBuf =
                                select_account_menu().expect("Failed to select account");
                            let account_config: AccountConfig = AccountConfig {
                                network: shared_args.network,
                                address: shared_args.address,
                                private_key: None,
                                path: Some(accout_path),
                            };
                            let account: Account = Account::new(account_config).unwrap();

                            let account_output: String = account.show_account()?;
                            println!("{}", account_output);
                        } else {
                            if shared_args.network.is_some() && shared_args.address.is_some() {
                                let account: Account = Account::default();
                                let account_output: String = account.show_account()?;
                                println!("{}", account_output);
                            } else if shared_args.network.is_some() && shared_args.address.is_none()
                            {
                                let account: Account = Account::default();
                                let account_output: String = account.show_account()?;
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
                            let account_config: AccountConfig = AccountConfig {
                                network: shared_args.network,
                                address: shared_args.address,
                                private_key: None,
                                path: Some(account_path),
                            };
                            let account: Account = Account::new(account_config).unwrap();
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

fn parse_deploy_args(
    deploy_args: Option<Vec<String>>,
    contracts_to_deploy: Vec<String>,
) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
    if deploy_args.is_some() && contracts_to_deploy.len() > 0 {
        let mut contract_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut arg_names: Vec<String> = Vec::new();
        let mut current_args: Vec<Vec<String>> = Vec::new();
        let mut sub_vector: Vec<String> = Vec::new();
        let mut current_args_index: usize = 0;

        for entry in deploy_args.unwrap().iter() {
            // iterate through args and if an arg is a contract name, split there and take the next set of args to the next contract name
            if contracts_to_deploy.contains(entry) {
                arg_names.push(entry.clone());
                current_args_index += 1;
                if sub_vector.len() > 0 {
                    current_args.push(sub_vector.clone());
                }
                sub_vector.clear();
                continue;
            } else if current_args_index > 0 {
                sub_vector.push(entry.clone());
                continue;
            } else {
                return Err("Contract name not found".into());
            }
        }
        if sub_vector.len() > 0 {
            current_args.push(sub_vector.clone());
        }
        for (index, arg_name) in arg_names.iter().enumerate() {
            contract_map.insert(arg_name.clone(), current_args[index].clone());
        }
        return Ok(contract_map);
    } else {
        return Err("No deploy args provided".into());
    }
}
