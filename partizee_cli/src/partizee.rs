use clap::Parser;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::commands::user_profile::{Profile, ProfileConfig};
use crate::utils::pbc_commands::{pbc_create_new_account, pbc_create_new_wallet};
use crate::utils::utils::{assert_partizee_project, get_address_from_pk};

use crate::commands::compile::ProjectCompiler;
use crate::commands::deploy::{DeployConfigs, Deployer, DeploymentWithProfile};
use crate::commands::new::{NewProject, ProjectConfig};
use crate::utils::clap_cli::{Arguments, Commands, ProfileSubcommands};
use crate::utils::fs_nav::{get_all_contract_names, id_pbc_path};
use crate::utils::menus::{
    compile_menu, create_new_pbc_account_menu, create_new_wallet_menu, deploy_menu,
    new_project_menu, select_pk_menu,
};
use crate::utils::constants::DEFAULT_NETWORK;

#[allow(unused_variables, unused_assignments)]
pub fn partizee() -> Result<(), Box<dyn std::error::Error>> {
    let partizee_cli: Arguments = Arguments::parse();
    match partizee_cli.commands {
        Commands::New {
            interactive,
            name,
            output_dir,
            zero_knowledge, // for future use
        } => {
            let new_project: NewProject;
            let mut interactive = interactive;
            // if all args are empty open interactive menu
            if !interactive && name.is_none() && output_dir.is_none() {
                interactive = true;
            }
            if interactive {
                let menu_args: ProjectConfig = new_project_menu(name, output_dir)?;
                new_project = NewProject::new(menu_args)?;
            } else {
                new_project = NewProject::new(ProjectConfig {
                    name: name.expect("must provide name for new project"),
                    output_dir: output_dir,
                })?;
            }
            // Pass zero_knowledge as needed
            new_project.create_new_project()?;
        }
        Commands::Compile {
            interactive,
            path,
            files_to_compile,
            build_args,
            additional_args,
        } => {
            assert_partizee_project()?;

            // create a new ProjectCompiler with the provided args
            let compile_args: ProjectCompiler = ProjectCompiler {
                path: path.clone(),
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
            pk_path,
        } => {
            assert_partizee_project()?;
            // check if the project is compiled

            let mut use_interactive: bool = interactive;
            // if all args are empty open interactive menu
            if !interactive
                && custom_net.is_none()
                && contract_names.is_none()
                && deploy_args.is_none()
                && pk_path.is_none()
            {
                use_interactive = true;
            }
            let mut deployer: DeploymentWithProfile;

            // if no contracts are provided, get all contract names from the project
            let mut contracts_to_deploy: Option<Vec<String>> = None;
            // get list of all contract names

            if contract_names.is_none() {
                contracts_to_deploy = get_all_contract_names();
                if contracts_to_deploy.is_none()
                    || contracts_to_deploy
                        .as_ref()
                        .unwrap_or(&Vec::new())
                        .is_empty()
                {
                    return Err("No contracts found in project, if you have contracts in your project, please compile by running `partizee compile`".into());
                }
            } else {
                contracts_to_deploy = contract_names;
            }

            let mut deployer_args_hashmap: Option<HashMap<String, Vec<String>>> = None;
            let parsed_deploy_args: Option<HashMap<String, Vec<String>>> = parse_deploy_args(
                deploy_args,
                contracts_to_deploy.as_ref().unwrap_or(&Vec::new()).clone(),
            );
            if contracts_to_deploy.is_some() {
                deployer_args_hashmap = parsed_deploy_args;
            } else {
                deployer_args_hashmap = None;
            }
            // create a new DeployConfigs with the provided args
            let config = DeployConfigs {
                network: custom_net,
                contract_names: contracts_to_deploy.unwrap_or(Vec::new()),
                deployer_args: deployer_args_hashmap,
                path_to_pk: pk_path.clone().map(|path| PathBuf::from(path)),
            };
            // if interactive, get options from interactive menu and pass deployer_args as needed
            if interactive {
                let menu_args: DeployConfigs = deploy_menu(config)?;
                let deployer_args: Deployer = Deployer {
                    network: menu_args.network.unwrap_or(DEFAULT_NETWORK.to_string()),
                    contract_names: menu_args.contract_names,
                    deployer_args: menu_args.deployer_args.unwrap_or(HashMap::new()),
                    path_to_pk: menu_args.path_to_pk.unwrap_or(PathBuf::from("")),
                };

                deployer = DeploymentWithProfile::new(deployer_args);
            } else {
                let final_pk_path: PathBuf;
                if config.path_to_pk.is_none() {
                    let pk_path: PathBuf = select_pk_menu()?;
                    final_pk_path = pk_path;
                } else {
                    // if passed in path is a file, use it, otherwise select a new account
                    if config.path_to_pk.as_ref().unwrap().is_file() {
                        final_pk_path = config.path_to_pk.clone().unwrap();
                    } else {
                        let pk_path: PathBuf = select_pk_menu()?;
                        final_pk_path = pk_path;
                    }
                }
                let deployer_args: Deployer = Deployer {
                    network: config.network.unwrap_or(DEFAULT_NETWORK.to_string()),
                    contract_names: config.contract_names,
                    deployer_args: config.deployer_args.unwrap_or(HashMap::new()),
                    path_to_pk: final_pk_path,
                };
                deployer = DeploymentWithProfile::new(deployer_args);
            }
            println!("Deploying contracts with args: {:#?}", deployer);
            let result = deployer.deploy_contracts();
            if !result.is_ok() {
                eprintln!("Contracts deployment failed");
            }
        }
        Commands::Profile { commands } => match commands {
            ProfileSubcommands::ProfileCreate { shared_args } => {
                let mut interactive: bool = shared_args.interactive;
                if shared_args.network.is_none() {
                    interactive = true;
                }
                let wallet_exists: bool = id_pbc_path().is_some();
                if !wallet_exists {
                    let network: String = create_new_wallet_menu()?;
                    if network.len() > 0 {
                        pbc_create_new_wallet(&network)?;
                    } else {
                        return Err("No wallet created".into());
                    }
                }
                if interactive {
                    let create_pbc_account: String = create_new_pbc_account_menu()?;
                    if create_pbc_account.len() > 0 {
                        // check if wallet already exists
                        pbc_create_new_account(&create_pbc_account)?;
                    }
                } else {
                    let network: String = shared_args.network.unwrap_or("testnet".to_string());
                    pbc_create_new_account(&network)?;
                }
            }
            ProfileSubcommands::ProfileShow { shared_args } => {
                if shared_args.interactive {
                    let accout_path: PathBuf = select_pk_menu().expect("Failed to select account");
                    let account_config: ProfileConfig = ProfileConfig {
                        network: shared_args.network,
                        address: shared_args.address,
                        private_key: None,
                        path_to_pk: Some(accout_path),
                    };
                    let account: Profile = Profile::new(account_config).unwrap();

                    let account_output: String = account.show_account()?;
                    println!("{}", account_output);
                } else {
                    // Create account from provided args or use default
                    let account_config = ProfileConfig {
                        network: shared_args.network,
                        address: shared_args.address,
                        private_key: None,
                        path_to_pk: None,
                    };

                    match Profile::new(account_config) {
                        Ok(account) => {
                            let account_output = account.show_account()?;
                            println!("{}", account_output);
                        }
                        Err(_) => println!("No account found"),
                    }
                }
            }
            ProfileSubcommands::ProfileMintGas { shared_args } => {
                let mut interactive: bool = shared_args.interactive;
                if shared_args.address.is_none()
                    && shared_args.network.is_none()
                    && shared_args.private_key.is_none()
                    && shared_args.path.is_none()
                {
                    interactive = true;
                }

                if interactive {
                    let pk_path: PathBuf = select_pk_menu().expect("Failed to select account");
                    let account_config: ProfileConfig = ProfileConfig {
                        network: shared_args.network,
                        address: shared_args.address,
                        private_key: None,
                        path_to_pk: Some(pk_path),
                    };
                    let account: Profile = Profile::new(account_config).unwrap();
                    account.mint_gas()?;
                } else {
                    let mut address: Option<String> = shared_args.address;
                    let network: Option<String> =
                        Some(shared_args.network.unwrap_or("testnet".to_string()));
                    let mut private_key: Option<String> = shared_args.private_key;
                    let mut pk_path: Option<PathBuf> =
                        shared_args.path.map(|path| PathBuf::from(path));

                    match (address.is_some(), private_key.is_some(), pk_path.is_some()) {
                        (false, true, false) => {
                            address = Some(get_address_from_pk(&private_key.clone().unwrap())?);
                        }
                        (true, false, false) => {
                            return Err("Cannot derive private key from address alone".into());
                        }
                        (false, false, false) => {
                            pk_path = Some(select_pk_menu().expect("Failed to select account"));
                            private_key = Some(fs::read_to_string(&pk_path.clone().unwrap())?);
                            address = Some(get_address_from_pk(&private_key.clone().unwrap())?);
                        }
                        (_, _, true) => {
                            private_key = Some(fs::read_to_string(&pk_path.clone().unwrap())?);
                            address = Some(get_address_from_pk(&private_key.clone().unwrap())?);
                        }
                        _ => {
                            return Err("Invalid arguments".into());
                        }
                    }
                    let account: Profile = Profile::new(ProfileConfig {
                        network: network.clone(),
                        address: address.clone(),
                        private_key: private_key.clone(),
                        path_to_pk: pk_path.clone(),
                    })?;
                    account.mint_gas()?;
                }
            }
        },
    }
    Ok(())
}

fn parse_deploy_args(
    deploy_args: Option<Vec<String>>,
    contracts_to_deploy: Vec<String>,
) -> Option<HashMap<String, Vec<String>>> {
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
                return None;
            }
        }

        if sub_vector.len() > 0 {
            current_args.push(sub_vector.clone());
        }
        for (index, arg_name) in arg_names.iter().enumerate() {
            contract_map.insert(arg_name.to_lowercase(), current_args[index].clone());
        }
        return Some(contract_map);
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_deploy_args() {
        let deploy_args: Option<Vec<String>> = Some(vec![
            "contract1".to_string(),
            "address1".to_string(),
            "address1b".to_string(),
            "contract2".to_string(),
            "address2".to_string(),
            "address2b".to_string(),
        ]);
        let contracts_to_deploy: Vec<String> =
            vec!["contract1".to_string(), "contract2".to_string()];
        let result: Option<HashMap<String, Vec<String>>> =
            parse_deploy_args(deploy_args, contracts_to_deploy);
        assert!(result.is_some());
        let contract_map: HashMap<String, Vec<String>> = result.unwrap();
        assert_eq!(contract_map.len(), 2);
        assert_eq!(contract_map.get("contract1").unwrap().len(), 2);
        assert_eq!(contract_map.get("contract2").unwrap().len(), 2);
    }
}
