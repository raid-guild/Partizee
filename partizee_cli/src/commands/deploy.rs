use crate::commands::user_profile::Profile;
use crate::utils::constants::DEFAULT_NETWORK;
use crate::utils::fs_nav::{
    find_dir, find_files_with_extension, find_paths_with_name, find_workspace_root,
    get_all_contract_names,
};
use crate::utils::utils::load_account_from_pk_file;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

/// Configuration for deploying Partisia Blockchain contracts
/// 
/// # Fields
/// * `contract_names` - List of contract names to deploy
/// * `network` - Optional network to deploy to (e.g. testnet, mainnet)
/// * `deployer_args` - Optional map of contract names to their deployment arguments
/// * `path_to_pk` - Optional path to private key file
#[derive(Debug, Clone)]
pub struct DeployConfigs {
    pub contract_names: Vec<String>,
    pub network: Option<String>,
    pub deployer_args: Option<HashMap<String, Vec<String>>>,
    pub path_to_pk: Option<PathBuf>,
}

/// Represents a deployed contract with its metadata
/// 
/// # Fields
/// * `name` - Name of the deployed contract
/// * `address` - Blockchain address where contract was deployed
/// * `args` - Arguments used during deployment
/// * `timestamp` - Unix timestamp of deployment
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Deployment {
    pub name: String,
    pub address: String,
    pub args: Vec<String>,
    pub timestamp: String,
}

/// Configuration for deploying contracts with network and account details
/// 
/// # Fields
/// * `network` - Network to deploy to
/// * `contract_names` - List of contracts to deploy
/// * `deployer_args` - Map of contract names to their deployment arguments
/// * `path_to_pk` - Path to private key file
#[derive(Debug, Clone)]
pub struct Deployer {
    pub network: String,
    pub contract_names: Vec<String>,
    pub deployer_args: HashMap<String, Vec<String>>,
    pub path_to_pk: PathBuf,
}

/// Combines deployment configuration with account profile
/// 
/// # Fields
/// * `deploy_configs` - Deployment configuration
/// * `account` - Account profile for deployment
#[derive(Debug, Clone)]
pub struct DeploymentWithProfile {
    deploy_configs: Deployer,
    account: Profile,
}

impl Default for DeployConfigs {
    /// Creates default deployment configuration
    /// Uses all available contracts and default network
    fn default() -> Self {
        let all_contract_names: Option<Vec<String>> = get_all_contract_names();
        Self {
            contract_names: all_contract_names.unwrap_or(Vec::new()),
            network: Some(DEFAULT_NETWORK.to_string()),
            deployer_args: None,
            path_to_pk: None,
        }
    }
}

// default deployment with account, selects either the first account found or creates a new account if none are found
impl Default for DeploymentWithProfile {
    /// Creates default deployment with account
    /// Selects first available account or creates new one if none found
    fn default() -> Self {
        let account: Profile = Profile::default();
        let mut deploy_project: DeployConfigs = DeployConfigs::default();
        deploy_project.path_to_pk = Some(account.path_to_pk.clone());
        let all_contract_names: Option<Vec<String>> = get_all_contract_names();

        let deployer: Deployer = Deployer {
            network: deploy_project
                .network
                .clone()
                .unwrap_or(DEFAULT_NETWORK.to_string()),
            contract_names: all_contract_names.unwrap_or(Vec::new()),
            deployer_args: deploy_project
                .deployer_args
                .clone()
                .unwrap_or(HashMap::new()),
            path_to_pk: deploy_project.path_to_pk.clone().expect("No account found"),
        };
        Self {
            deploy_configs: deployer,
            account: account,
        }
    }
}

#[allow(dead_code)]
impl DeploymentWithProfile {
    /// Creates new deployment with specified configuration
    /// 
    /// # Arguments
    /// * `deploy_config` - Deployment configuration
    /// 
    /// # Returns
    /// * `DeploymentWithProfile` - New deployment instance
    pub fn new(deploy_config: Deployer) -> Self {
        let deployment_account: Profile =
            load_account_from_pk_file(&deploy_config.path_to_pk, &deploy_config.network)
                .expect("Failed to load account");
        Self {
            deploy_configs: deploy_config,
            account: deployment_account,
        }
    }

    /// Deploys all specified contracts to the blockchain
    /// 
    /// Handles finding and loading contract files (.pbc, .abi, .wasm, .zkwa)
    /// Saves deployment results to deployment-latest.json
    /// 
    /// # Returns
    /// * `Result<(), Box<dyn std::error::Error>>` - Ok if all deployments succeed
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
        let names_set: HashSet<String> = names.iter().map(|name| name.to_lowercase()).collect();
        names = names_set.into_iter().collect();

        if names.is_empty() {
            names = get_all_contract_names().unwrap_or(Vec::new());
            // get all contract abis, pbc, wasm, and zkwa files
            contract_pbc_set = find_files_with_extension(&path_to_contracts, "pbc")
                .into_iter()
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

        let contract_pbc_map: HashMap<String, PathBuf> = Self::build_contract_file_map(contract_pbc_paths.clone());
        let contract_abi_map: HashMap<String, PathBuf> = Self::build_contract_file_map(contract_abi_paths.clone());
        let contract_wasm_map: HashMap<String, PathBuf> = Self::build_contract_file_map(contract_wasm_paths.clone());
        let contract_zkwa_map: HashMap<String, PathBuf> = Self::build_contract_file_map(contract_zkwa_paths.clone());

        let contract_args_hashmap: HashMap<String, Vec<String>> =
            self.deploy_configs.deployer_args.clone();

        let mut deployments: Vec<Deployment> = Vec::new();

        for (_, name) in names.iter().enumerate() {
            let name_lowercase = name.to_lowercase();
            // get name of current contract
            let contract_args: Vec<String> =
                contract_args_hashmap.get(&name_lowercase).cloned().unwrap_or_default();
            if contract_args.len() > 0 {
                println!("Deploying {} with args: {:?}", &name, &contract_args);
            }

            let contract_pbc_path= contract_pbc_map.get(&name_lowercase).cloned();
            let contract_abi_path= contract_abi_map.get(&name_lowercase).cloned();
            let contract_wasm_path= contract_wasm_map.get(&name_lowercase).cloned();
            let contract_zkwa_path= contract_zkwa_map.get(&name_lowercase).cloned();
            




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
                let deployment = result.unwrap_or_else(|e| {
                    panic!("Error deploying contract {}: {:?}", name, e);
                });
                deployments.push(deployment);
            }
        }

        save_deployments(deployments, &project_root)?;

        Ok(())
    }

    /// Deploys a single contract to the blockchain
    /// 
    /// # Arguments
    /// * `name` - Name of contract to deploy
    /// * `contract_pbc_path` - Optional path to .pbc file
    /// * `contract_abi_path` - Optional path to .abi file
    /// * `contract_wasm_path` - Optional path to .wasm file
    /// * `contract_zkwa_path` - Optional path to .zkwa file
    /// * `args` - Deployment arguments
    /// 
    /// # Returns
    /// * `Result<Deployment>` - Deployment result with contract address
    pub fn deploy_contract(
        &mut self,
        name: &str,
        contract_pbc_path: Option<PathBuf>,
        contract_abi_path: Option<PathBuf>,
        contract_wasm_path: Option<PathBuf>,
        contract_zkwa_path: Option<PathBuf>,
        args: Vec<String>,
    ) -> Result<Deployment, Box<dyn std::error::Error>> {
        println!("deploy_contract: {:#?}", contract_pbc_path);
        println!("args: {:#?}", args);
        println!("name: {:#?}", name);
        // cargo partisia-contract transaction deploy --gas 10000000 --privatekey YourProfileFile.pk your_compiled_contract_file.pbc + contract inputs separated by spaces (strings in quotes)
        let private_key_path: PathBuf = self.account.path_to_pk.clone();
        assert!(self.deploy_configs.network.len() > 0);
        let network_command: String = format!("--net={}", &self.deploy_configs.network);
        let mut command: Command = Command::new("cargo");
        command
            .arg("pbc")
            .arg("transaction")
            .arg("-v")
            .arg(&network_command)
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

        command.args(&args);
        println!("Deploying {} to {}.", &name, &self.deploy_configs.network);
        let deployment_tx: Output = command.output()?;

        if deployment_tx.status.success() {
            let output_str = String::from_utf8_lossy(&deployment_tx.stdout);
            let address = output_str
                .split(":")
                .nth(1)
                .unwrap_or("")
                .split("\n")
                .nth(0)
                .unwrap_or("")
                .trim()
                .to_string();
            if address.is_empty() {
                return Err("Failed to get address from deployment output".into());
            }
            let timestamp: String = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_else(|_| {
                    panic!("Failed to get timestamp");
                })
                .as_secs()
                .to_string();
            let deployment = Deployment {
                name: name.to_string(),
                address,
                args,
                timestamp,
            };
            println!(
                "✅ Successfully deployed contract '{}' to '{}' at address: {}",
                name, &self.deploy_configs.network, deployment.address
            );
            return Ok(deployment);
        } else {
            let stdout = String::from_utf8_lossy(&deployment_tx.stdout);
            let stderr = String::from_utf8_lossy(&deployment_tx.stderr);
            let status = deployment_tx.status;
            eprintln!("❌ Failed to deploy contract:");
            eprintln!("Status: {}", status);
            eprintln!("Stdout: {}", stdout);
            eprintln!("Stderr: {}", stderr);
            return Err("Failed to deploy contract".into());
        }
    }
    /// Gets deployment arguments for a specific contract
    /// 
    /// # Arguments
    /// * `name` - Name of contract
    /// 
    /// # Returns
    /// * `Option<Vec<String>>` - Deployment arguments if found
    pub fn get_deployer_args_for_name(&self, name: &str) -> Option<Vec<String>> {
        let name_lowercase = name.to_lowercase();
        self.deploy_configs.deployer_args.get(&name_lowercase).cloned()
    }

    /// Builds a map of contract names to their file paths
    /// 
    /// # Arguments
    /// * `paths` - Vector of contract file paths
    /// 
    /// # Returns
    /// * `HashMap<String, PathBuf>` - Map of lowercase contract names to paths
    pub fn build_contract_file_map(paths: Vec<PathBuf>) -> HashMap<String, PathBuf> {
        let mut map = HashMap::new();
        for path in paths {
            if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()).map(|s| s.to_lowercase()) {
                map.insert(file_stem.to_string(), path.clone());
            }
        }
        map
    }

}

/// Saves deployment results to JSON file
/// 
/// Creates deployment history by renaming existing deployment-latest.json
/// 
/// # Arguments
/// * `deployments` - Vector of deployment results
/// * `project_root` - Root directory of project
/// 
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Ok if save succeeds
fn save_deployments(
    deployments: Vec<Deployment>,
    project_root: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    // write deployment to target directory
    let target_dir: PathBuf = find_dir(&project_root, "target/wasm32-unknown-unknown/release")
        .unwrap_or_else(|| {
            panic!("Failed to find target directory");
        });
    // pop the release directory and join the deployments directory
    let deployment_dir: PathBuf = target_dir.parent().unwrap().join("deployments");
    if !deployment_dir.exists() {
        fs::create_dir_all(&deployment_dir).unwrap();
    }

    // get current deployment-latest.json and rename it to deployment-<timestamp>.json
    let latest_path: PathBuf = deployment_dir.join("deployment-latest.json");
    if latest_path.exists() {
        let timestamp: String = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
        let new_filename: PathBuf = deployment_dir.join(format!("deployment-{}.json", timestamp));
        fs::rename(&latest_path, new_filename).unwrap_or_else(|e| {
            eprintln!("Failed to rename deployment-latest.json: {}", e);
            return ();
        });
    }

    if deployments.len() > 0 {
        let deployments_json: String = serde_json::to_string(&deployments).unwrap_or_else(|e| {
            eprintln!("Failed to write deployment: {}", e);
            return "".to_string();
        });
        fs::write(&latest_path, deployments_json).unwrap_or_else(|e| {
            eprintln!("Failed to write deployment: {}", e);
            return ();
        });
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::utils::setup_test_environment;

    fn cleanup(original_dir: PathBuf) {
        std::env::set_current_dir(original_dir).unwrap();
    }
    #[test]
    fn test_create_default_deployment_with_account() {
        let (temp_dir, temp_path, original_dir) = setup_test_environment();
        std::env::set_current_dir(&temp_path).unwrap();
        let path_to_pk: PathBuf = temp_path.join("00d277aa1bf5702ab9fc690b04bd68b5a981095530.pk");
        // get pk files
        // create new project
        let deployment_with_account: DeploymentWithProfile = DeploymentWithProfile::default();
        assert!(deployment_with_account.deploy_configs.path_to_pk.is_file());
        assert_eq!(
            deployment_with_account.account.path_to_pk.is_file().clone(),
            true
        );
        assert_eq!(
            deployment_with_account
                .account
                .path_to_pk
                .extension()
                .unwrap(),
            "pk"
        );
        assert_eq!(
            deployment_with_account
                .account
                .path_to_pk
                .clone()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
            path_to_pk.file_name().unwrap().to_str().unwrap()
        );
        assert_eq!(deployment_with_account.account.address.len(), 42);
        assert_eq!(deployment_with_account.account.private_key.len(), 64);
        assert_eq!(deployment_with_account.account.network, "testnet");
        assert_eq!(
            temp_dir
                .path()
                .join("00d277aa1bf5702ab9fc690b04bd68b5a981095530.pk")
                .exists(),
            true
        );
        cleanup(original_dir);
    }

    #[test]
    fn test_build_contract_file_map() {
        let (_, temp_path, original_dir) = setup_test_environment();
        let _ = std::env::set_current_dir(&temp_path);
        let contract_dir: PathBuf = temp_path.join("rust/contracts");
        let _ =fs::create_dir_all(&contract_dir);
        let contract1_path: PathBuf = contract_dir.join("Contract1.pbc");
        let contract2_path: PathBuf = contract_dir.join("Contract2.pbc");
        let _ = fs::write(&contract1_path, "");
        let _ = fs::write(&contract2_path, "");
        let paths: Vec<PathBuf> = vec![contract1_path.clone(), contract2_path.clone()];
        let map: HashMap<String, PathBuf> = DeploymentWithProfile::build_contract_file_map(paths);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("contract1").unwrap(), &contract1_path.clone());
        assert_eq!(map.get("contract2").unwrap(), &contract2_path.clone());
        cleanup(original_dir);
    }

    #[test]
    #[allow(unused_variables)]
    fn test_get_deployer_args() {
        let (temp_dir, temp_path, original_dir) = setup_test_environment();
        let _ = std::env::set_current_dir(&temp_path);
        let contract_dir: PathBuf = temp_path.join("rust/contracts");
        let _ =fs::create_dir_all(&contract_dir);
        let contract1_path: PathBuf = contract_dir.join("Contract1.pbc");
        let contract2_path: PathBuf = contract_dir.join("Contract2.pbc");
        let _ = fs::write(&contract1_path, "");
        let _ = fs::write(&contract2_path, "");
        // create a deployer args hashmap
        let mut deployer_args: HashMap<String, Vec<String>> = HashMap::new();
        deployer_args.insert("contract1".to_string(), vec!["arg1".to_string(), "arg2".to_string()]);
        deployer_args.insert("contract2".to_string(), vec!["arg3".to_string(), "arg4".to_string()]);
        // create a deployer
        let pk_path = temp_path.join("00d277aa1bf5702ab9fc690b04bd68b5a981095530.pk").canonicalize();
        assert!(pk_path.as_ref().unwrap().exists(), "PK file does not exist at {:?}", pk_path.as_ref().unwrap());
        let deployer: Deployer = Deployer {
            network: "testnet".to_string(),
            contract_names: vec!["contract1".to_string(), "contract2".to_string()],
            deployer_args,
            path_to_pk: pk_path.unwrap(),
        };
        let deployment_with_account: DeploymentWithProfile = DeploymentWithProfile::new(deployer);
        let args: Vec<String> = deployment_with_account.get_deployer_args_for_name("Contract1").unwrap();
        assert_eq!(args, vec!["arg1".to_string(), "arg2".to_string()]);
        let args: Vec<String> = deployment_with_account.get_deployer_args_for_name("Contract2").unwrap();
        assert_eq!(args, vec!["arg3".to_string(), "arg4".to_string()]);
        let paths: Vec<PathBuf> = vec![contract1_path.clone(), contract2_path.clone()];
        let map: HashMap<String, PathBuf> = DeploymentWithProfile::build_contract_file_map(paths);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("contract1").unwrap(), &contract1_path.clone());
        assert_eq!(map.get("contract2").unwrap(), &contract2_path.clone());
        cleanup(original_dir);
    }
    
}
