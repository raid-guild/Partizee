// cargo pbc transaction deploy --net="testnet" target/wasm32-unknown-unknown/release/ballot.zkwa
// '[YES, NO]' "Does this work?" "Let's see..." --abi=target/wasm32-unknown-unknown/release/ballot.abi
// --pk ./Account-A.pk --gas=10000000

use crate::commands::account::Account;
use crate::utils::constants::DEFAULT_NETWORK;
use crate::utils::utils::{
    find_paths_with_extension, find_paths_with_name, load_account_from_pk_file, print_error,
    print_output,
};
use pbc_abi::abi_model::{ContractAbi, FnAbi};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::{Command, Output};
#[derive(Debug, Clone)]
pub struct DeployConfigs {
    pub contract_names: Option<Vec<String>>,
    pub network: Option<String>,
    pub deployer_args: Option<HashMap<String, Vec<String>>>,
    pub path_to_account: Option<PathBuf>,
}

pub struct DeploymentWithAccount {
    deploy_configs: DeployConfigs,
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

impl Default for DeploymentWithAccount {
    fn default() -> Self {
        let account: Account = Account::default();
        let mut deploy_project: DeployConfigs = DeployConfigs::default();
        deploy_project.path_to_account = Some(account.path.clone());
        Self {
            deploy_configs: deploy_project,
            account: account,
        }
    }
}

impl DeploymentWithAccount {
    pub fn new(deploy_config: DeployConfigs, account_path: Option<PathBuf>) -> Self {
        let deployment_account: Account = if account_path.is_some() {
            load_account_from_pk_file(
                account_path.as_ref().unwrap(),
                &deploy_config
                    .network
                    .as_ref()
                    .unwrap_or(&DEFAULT_NETWORK.to_string()),
            )
            .unwrap()
        } else {
            Account::default()
        };

        Self {
            deploy_configs: deploy_config.clone(),
            account: deployment_account,
        }
    }

    /// deploy all contracts in the contracts directory
    /// if no contract names are provided, deploy all contracts in the contracts directory
    /// if contract names are provided, deploy only the contracts with the given names
    /// if deployer args are provided, deploy the contracts with the given names and args
    /// if no deployer args are provided, deploy the contracts with the default arguments
    /// if no network is provided, use the default network
    /// if no account is provided, use the default account
    pub fn deploy_contracts(&mut self) /*-> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>>*/
    {
        println!("Deploying contracts...");
        let network: String = self.deploy_configs.network.clone().unwrap_or(DEFAULT_NETWORK.to_string());
        let project_root: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        println!("Network: {:?}", network);
        let names: Vec<String> = self
            .deploy_configs
            .contract_names
            .clone()
            .unwrap_or(Vec::new());
        let mut path_to_contracts: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path_to_contracts.push("target/wasm32-unknown-unknown/release");

        let mut contract_pbc_paths: Vec<PathBuf> = Vec::new();
        let mut contract_abi_paths: Vec<PathBuf> = Vec::new();
        let mut contract_wasm_paths: Vec<PathBuf> = Vec::new();
        let mut contract_zkwa_paths: Vec<PathBuf> = Vec::new();
        if names.is_empty() {
            contract_pbc_paths = find_paths_with_extension(&path_to_contracts, "pbc");
            // contract_abi_paths = find_paths_with_extension(&path_to_contracts, "abi");
            // contract_wasm_paths = find_paths_with_extension(&path_to_contracts, "wasm");
            // contract_zkwa_paths = find_paths_with_extension(&path_to_contracts, "zkwa");
        } else {
            for name in names {
                let all_contract_paths = find_paths_with_name(&path_to_contracts, &name);
                // filter for only pbc files
                contract_pbc_paths.push(
                    all_contract_paths
                        .iter()
                        .filter(|path| path.extension().unwrap_or_default() == "pbc")
                        .cloned()
                        .collect(),
                );
                // filter for only abi files
                contract_abi_paths.push(
                    all_contract_paths
                        .iter()
                        .filter(|path| path.extension().unwrap_or_default() == "abi")
                        .cloned()
                        .collect(),
                );
                // filter for only wasm files
                contract_wasm_paths.push(
                    all_contract_paths
                        .iter()
                        .filter(|path| path.extension().unwrap_or_default() == "wasm")
                        .cloned()
                        .collect(),
                );
                // filter for only zkwa files
                contract_zkwa_paths.push(
                    all_contract_paths
                        .iter()
                        .filter(|path| path.extension().unwrap_or_default() == "zkwa")
                        .cloned()
                        .collect(),
                );
            }
        }
        let contract_args: HashMap<String, Vec<String>> = self
            .deploy_configs
            .deployer_args
            .clone()
            .unwrap_or(HashMap::new());
        println!("Contract args: {:?}", contract_args);
        // hashmap key is contract name
        // hashmap value is vector of arguments
        // if self.deploy_configs.deployer_args.is_some() {
        //     for (contract_name, args) in self.deploy_configs.deployer_args.as_ref().unwrap().iter() {
        //         // search contracts folder for folder with name contract_name
        //         let path_to_contract: PathBuf = project_root.push(PathBuf::from("rust/contracts/").push(PathBuf::from(contract_name))).push("lib.rs");

        //         let contract_abi = ContractAbi::from_file(path_to_contract).unwrap();
        //         let init_fn: Option<&FnAbi> = contract_abi.functions.iter().find(|f| f.fn_kind == FunctionKind::Init);
        //         // check if contract has initialize function
        //         if init_fn.is_some() {
        //            // check that provided args match the initialize function inputs
        //            println!("Contract: {:?}, Args: {:?}", contract_name, args);
        //         }
        //         // check if contract has initialize function with inputs
        //         if contract_abi.functions.iter().find(|fn_abi| fn_abi.name == "initialize").unwrap().inputs.is_empty() {
        //         let init_fn: FnAbi = contract_abi.functions.iter().find(|fn_abi| fn_abi.name == "initialize").unwrap();
        //         if init_fn.inputs.is_empty() {
        //             return Err("No inputs provided for contract".into());
        //             }
        //         }
        //     }
        // }

        // if contract_pbc_paths.is_empty() {
        //     return Err("No contracts found".into());
        // }

        // let mut results: Vec<Vec<u8>> = Vec::new();
        // if &network == "testnet" {
        //     for contract_path in contract_paths {
        //         // check abi for initialization types

        //         let contract_deploy_args: Vec<String> = self.deploy_configs.deployer_args.clone().unwrap_or(Vec::new());
        //         results.push(self.deploy_contract(Some(contract_path), None, None, None, contract_deploy_args)?);
        //     }
        //     return Ok(results);
        // } else if &network == "mainnet" {
        //     for contract_pbc in contract_pbcs {
        //         let contract_name: &OsStr = contract_pbc.file_name().unwrap();
        //         let contract_deploy_args: Vec<String> =
        //             contract_deploy_args(&contract_name.to_str().unwrap())?;
        //         self.deploy_contract(Some(contract_pbc), None, None, None, contract_deploy_args)?;
        //     }
        //     return Ok(results);
        // } else {
        //     println!("Invalid network");
        //     return Err("Invalid network".into());
        // }
    }

    pub fn deploy_contract(
        &mut self,
        contract_name: String,
        contract_pbc_path: Option<PathBuf>,
        contract_abi_path: Option<PathBuf>,
        contract_wasm_path: Option<PathBuf>,
        contract_zkwa_path: Option<PathBuf>,
        args: Vec<String>,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // cargo partisia-contract transaction deploy --gas 10000000 --privatekey YourAccountFile.pk your_compiled_contract_file.pbc + contract inputs separated by spaces (strings in quotes)
        let private_key_path: PathBuf = self.account.path.clone();
        println!(
            "Command: cargo pbc transaction deploy --gas 10000000 --privatekey {:?} --abi{:?} {:?}",
            &private_key_path,
            contract_pbc_path.as_ref().unwrap(),
            args.join(" ")
        );

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
                // command.arg(zkwa);
                // If you need to add --abi here, do so
                // command.arg("--abi");
                // command.arg(abi);
            }
            (Some(pbc), None, None, None) => {
                command.arg(pbc.to_str().unwrap());
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
            return print_output("deploy_contract", &deployment_tx);
        } else {
            return print_error(&deployment_tx);
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[derive(Debug, Clone)]
//     struct DeployProjectTest {
//         account_name: Option<String>,
//     }
//     #[test]
//     fn test_deploy_contracts() {
//         // create new project

//         let deploy_config: DeployProject = DeployProject {
//             account_name: Some("test".to_string()),
//             project_root: find_workspace_root(),
//             contract_path: None,
//             network: None,
//             deployer_args: None,
//             account: None,
//         };
//         let mut deploy_project: DeployProject = DeployProject::new(deploy_config);
//         let result = deploy_project
//             .deploy_contracts(None)
//             .expect("Failed to deploy contracts");
//         println!("{:?}", result);
//     }
// }
