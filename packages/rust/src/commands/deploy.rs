use crate::commands::account::Account;
use crate::utils::constants::DEFAULT_NETWORK;
use crate::utils::fs_nav::{
    find_dir, find_files_with_extension, find_paths_with_name, find_workspace_root,
    get_all_contract_names,
};
use std::fs;
use serde::{Serialize, Deserialize};
use crate::utils::utils::{load_account_from_pk_file, print_error, print_output};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::process::{Command, Output};
#[derive(Debug, Clone)]
pub struct DeployConfigs {
    pub contract_names: Option<Vec<String>>,
    pub network: Option<String>,
    pub deployer_args: Option<HashMap<String, Vec<String>>>,
    pub path_to_account: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Deployment {
    pub name: String,
    pub address: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Deployer {
    pub network: String,
    pub contract_names: Vec<String>,
    pub deployer_args: HashMap<String, Vec<String>>,
    pub path_to_account: PathBuf,
}
#[derive(Debug, Clone)]
pub struct DeploymentWithAccount {
    deploy_configs: Deployer,
    account: Account,
}

impl Default for DeployConfigs {
    fn default() -> Self {
        Self {
            contract_names: None,
            network: Some(DEFAULT_NETWORK.to_string()),
            deployer_args: None,
            path_to_account: None,
        }
    }
}

// default deployment with account, selects either the first account found or creates a new account if none are found
impl Default for DeploymentWithAccount {
    fn default() -> Self {
        let account: Account = Account::default();
        let mut deploy_project: DeployConfigs = DeployConfigs::default();
        deploy_project.path_to_account = Some(account.path.clone());
        let deployer: Deployer = Deployer {
            network: deploy_project
                .network
                .clone()
                .unwrap_or(DEFAULT_NETWORK.to_string()),
            contract_names: get_all_contract_names().expect("No contracts found"),
            deployer_args: deploy_project
                .deployer_args
                .clone()
                .unwrap_or(HashMap::new()),
            path_to_account: deploy_project
                .path_to_account
                .clone()
                .expect("No account found"),
        };
        Self {
            deploy_configs: deployer,
            account: account,
        }
    }
}

#[allow(dead_code)]
impl DeploymentWithAccount {
    pub fn new(deploy_config: Deployer) -> Self {
        let deployment_account: Account =
            load_account_from_pk_file(&deploy_config.path_to_account, &deploy_config.network)
                .expect("Failed to load account");
        Self {
            deploy_configs: deploy_config,
            account: deployment_account,
        }
    }

    /// deploy all contracts in the contracts directory
    pub fn deploy_contracts(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let project_root: PathBuf = find_workspace_root().unwrap();

        let mut names: Vec<String> = self.deploy_configs.contract_names.clone();

        let path_to_contracts: PathBuf =
            find_dir(&project_root, "wasm32-unknown-unknown/release").unwrap();

        let mut contract_pbc_set: HashSet<PathBuf> = HashSet::new();
        let mut contract_abi_set: HashSet<PathBuf> = HashSet::new();
        let mut contract_wasm_set: HashSet<PathBuf> = HashSet::new();
        let mut contract_zkwa_set: HashSet<PathBuf> = HashSet::new();
        // filter repeat names
        let names_set: HashSet<String> = names.iter().map(|name| name.clone()).collect();
        names = names_set.into_iter().collect();

        if names.is_empty() {
            contract_pbc_set = find_files_with_extension(&path_to_contracts, "pbc")
                .into_iter()
                .collect();
            // trim pbc paths to just the name of the contract
            names = contract_pbc_set
                .iter()
                .map(|path| {
                    path.file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string()
                        .replace(".pbc", "")
                })
                .collect();
            contract_abi_set = find_files_with_extension(&path_to_contracts, "abi")
                .into_iter()
                .collect();
            contract_wasm_set = find_files_with_extension(&path_to_contracts, "wasm")
                .into_iter()
                .collect();
            contract_zkwa_set = find_files_with_extension(&path_to_contracts, "zkwa")
                .into_iter()
                .collect();
        } else {
            for name in &names {
                let all_contract_paths = find_paths_with_name(&path_to_contracts, name);
                // filter for only pbc files
                contract_pbc_set.insert(
                    all_contract_paths
                        .iter()
                        .filter(|path| path.extension().unwrap_or_default() == "pbc")
                        .cloned()
                        .collect(),
                );
                // filter for only abi files
                contract_abi_set.insert(
                    all_contract_paths
                        .iter()
                        .filter(|path| path.extension().unwrap_or_default() == "abi")
                        .cloned()
                        .collect(),
                );
                // filter for only wasm files
                contract_wasm_set.insert(
                    all_contract_paths
                        .iter()
                        .filter(|path| path.extension().unwrap_or_default() == "wasm")
                        .cloned()
                        .collect(),
                );
                // filter for only zkwa files
                contract_zkwa_set.insert(
                    all_contract_paths
                        .iter()
                        .filter(|path| path.extension().unwrap_or_default() == "zkwa")
                        .cloned()
                        .collect(),
                );
            }
        }

        // convert sets to vectors
        let contract_pbc_paths: Vec<PathBuf> = contract_pbc_set.into_iter().collect();
        let contract_abi_paths: Vec<PathBuf> = contract_abi_set.into_iter().collect();
        let contract_wasm_paths: Vec<PathBuf> = contract_wasm_set.into_iter().collect();
        let contract_zkwa_paths: Vec<PathBuf> = contract_zkwa_set.into_iter().collect();

        let contract_args_hashmap: HashMap<String, Vec<String>> =
            self.deploy_configs.deployer_args.clone();

        for (index, name) in names.iter().enumerate() {
            // get name of current contract
            let contract_args: Vec<String> =
                contract_args_hashmap.get(name).cloned().unwrap_or_default();

            let contract_pbc_path: Option<PathBuf> = contract_pbc_paths
                .get(index)
                .filter(|p: &&PathBuf| p.exists())
                .cloned();
            let contract_abi_path: Option<PathBuf> = contract_abi_paths
                .get(index)
                .filter(|p: &&PathBuf| p.exists())
                .cloned();

            let contract_wasm_path: Option<PathBuf> = contract_wasm_paths
                .get(index)
                .filter(|p: &&PathBuf| p.exists())
                .cloned();

            let contract_zkwa_path: Option<PathBuf> = contract_zkwa_paths
                .get(index)
                .filter(|p: &&PathBuf| p.exists())
                .cloned();

            let result = self.deploy_contract(
                name,
                contract_pbc_path,
                contract_abi_path,
                contract_wasm_path,
                contract_zkwa_path,
                contract_args,
            );

            if result.is_err() {
                eprintln!(
                    "Error deploying contract {}: {:?}",
                    name,
                    result.err().unwrap()
                );
            } else {
                let deployment = result.unwrap();
                save_deployment(deployment, &project_root)?;
                println!("Contract deployed: {}", name);
            }
        }

        Ok(())
    }

    pub fn deploy_contract(
        &mut self,
        name: &str,
        contract_pbc_path: Option<PathBuf>,
        contract_abi_path: Option<PathBuf>,
        contract_wasm_path: Option<PathBuf>,
        contract_zkwa_path: Option<PathBuf>,
        args: Vec<String>,
    ) -> Result<Deployment, Box<dyn std::error::Error>> {
        // cargo partisia-contract transaction deploy --gas 10000000 --privatekey YourAccountFile.pk your_compiled_contract_file.pbc + contract inputs separated by spaces (strings in quotes)
        let private_key_path: PathBuf = self.account.path.clone();

        let mut command: Command = Command::new("cargo");
        command
            .arg("pbc")
            .arg("transaction")
            .arg("deploy")
            .arg("--gas")
            .arg("10000000")
            .arg("--pk")
            .arg(&private_key_path);

        match (
            contract_pbc_path.as_ref(),
            contract_abi_path.as_ref(),
            contract_wasm_path.as_ref(),
            contract_zkwa_path.as_ref(),
        ) {
            (Some(pbc), Some(abi), None, None) => {
                command.arg(pbc);
                command.arg("--abi");
                command.arg(abi.to_str().unwrap());
            }
            (None, Some(abi), Some(wasm), None) => {
                command.arg(wasm);
                command.arg("--abi");
                command.arg(abi.to_str().unwrap());
            }
            (None, Some(abi), None, Some(zkwa)) => {
                command.arg(zkwa);
                command.arg("--abi");
                command.arg(abi.to_str().unwrap());
            }
            (Some(pbc), None, None, None) => {
                command.arg(pbc.to_str().unwrap());
            }
            (Some(pbc), Some(_abi), Some(_wasm), None) => {
                command.arg(pbc.to_str().unwrap());
            }

            (Some(pbc), None, Some(_wasm), None) => {
                command.arg(pbc.to_str().unwrap());
            }
            (Some(pbc), None, Some(_wasm), Some(_zkwa)) => {
                command.arg(pbc.to_str().unwrap());
            }
            (Some(pbc), None, None, Some(_zkwa)) => {
                command.arg(pbc.to_str().unwrap());
            }
            (Some(pbc), Some(_abi), Some(_wasm), Some(_zkwa)) => {
                command.arg(pbc.to_str().unwrap());
            }
            (None, Some(abi), Some(_wasm), Some(zkwa)) => {
                command.arg(zkwa.to_str().unwrap());
                command.arg("--abi");
                command.arg(abi.to_str().unwrap());
            }
            _ => {
                return Err("Need either pbc or wasm + abi or zkwa + abi paths provided".into());
            }
        }
        // log the final command

        command.args(&args);
        println!("Final command: {:?}", command);
        let deployment_tx: Output = command.output()?;

        if deployment_tx.status.success() {
            let output_str = String::from_utf8_lossy(&deployment_tx.stdout);
            let address = output_str.lines().nth(1).unwrap_or("").to_string();
            let deployment = Deployment { name: name.to_string(), address, args };
            return Ok(deployment);
        } else {
            return Err(format!("Failed to deploy contract: {:?}", deployment_tx).into());
        }
    }
}


fn save_deployment(deployment: Deployment, project_root: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
                 // write deployment to target directory
                 let target_dir: PathBuf = find_dir(&project_root, "target/wasm32-unknown-unknown/release").unwrap_or_else(|| {
                    panic!("Failed to find target directory");
                });
                // pop the release directory and join the deployments directory
                let deployment_dir: PathBuf = target_dir.parent().unwrap().join("deployments");
                if !deployment_dir.exists() {
                    fs::create_dir_all(&deployment_dir).unwrap();
                }
                // create filename from timestamp
                let timestamp: String = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string();
                let deployment_file: PathBuf = deployment_dir.join(format!("deployment-{}.json", timestamp));
                let deployment_json: String = serde_json::to_string(&deployment).unwrap_or_else(|e| {
                    eprintln!("Failed to serialize deployment: {}", e);
                    return "".to_string();
                });

                fs::write(deployment_file, deployment_json).unwrap_or_else(|e| {
                    eprintln!("Failed to write deployment: {}", e);
                    return ();
                });
                Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::fs_nav::get_pk_files;

    #[test]
    fn test_create_default_deployment_with_account() {
        // get pk files
        let pk_files: Vec<PathBuf> = get_pk_files();
        if pk_files.len() > 0 {
            // create new project
            let deployment_with_account: DeploymentWithAccount = DeploymentWithAccount::default();
            assert!(deployment_with_account
                .deploy_configs
                .path_to_account
                .is_file());
            assert_eq!(deployment_with_account.account.path.is_file().clone(), true);
            assert_eq!(
                deployment_with_account.account.path.extension().unwrap(),
                "pk"
            );
            assert_eq!(
                deployment_with_account
                    .account
                    .path
                    .clone()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap(),
                format!("{}.pk", deployment_with_account.account.address)
            );
            assert_eq!(deployment_with_account.account.address.len(), 42);
            assert_eq!(deployment_with_account.account.private_key.len(), 64);
            assert_eq!(deployment_with_account.account.network, "testnet");
        } else {
            println!("must create a new account");
        }
    }
}
