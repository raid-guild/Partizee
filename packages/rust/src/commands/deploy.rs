// cargo pbc transaction deploy --net="testnet" target/wasm32-unknown-unknown/release/ballot.zkwa
// '[YES, NO]' "Does this work?" "Let's see..." --abi=target/wasm32-unknown-unknown/release/ballot.abi
// --pk ./Account-A.pk --gas=10000000

use crate::commands::account::Account;
use crate::utils::menus::contract_deploy_args;
use crate::utils::utils::{
    default_save_path, find_paths_with_extension, find_wasm_release_folder, find_workspace_root,
    load_from_file, print_error, print_output,
};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::{Command, Output};

#[derive(Debug, Clone)]
pub struct DeployProject {
    pub project_root: Option<PathBuf>,
    pub contract_path: Option<PathBuf>,
    pub network: Option<String>,
    pub deployer_args: Option<Vec<String>>,
    pub account_name: Option<String>,
    pub account: Option<Account>,
}

impl Default for DeployProject {
    fn default() -> Self {
        Self {
            project_root: None,
            contract_path: None,
            network: None,
            deployer_args: None,
            account_name: None,
            account: None,
        }
    }
}

impl DeployProject {
    pub fn new(deploy_config: DeployProject) -> Self {
        let project_root: Option<PathBuf> = if deploy_config.project_root.is_some() {
            deploy_config.project_root
        } else {
            Some(env!("CARGO_MANIFEST_DIR").into())
        };

        let network: Option<String> = if deploy_config.network.is_some() {
            deploy_config.network
        } else {
            Some(String::from("testnet"))
        };

        let deployment_account: Option<Account> = if deploy_config.account_name.is_some() {
            let account_path =
                default_save_path(&deploy_config.account_name.as_ref().unwrap().to_string());
            println!("Account path: {:?}", account_path);
            if account_path.is_file() {
                load_from_file(Some(&account_path))
            } else {
                Some(Account::new(
                    deploy_config.account_name.as_deref(),
                    network.as_deref(),
                    None,
                    None,
                ))
            }
        } else {
            // look in .account folder for first account
            let account_files: Vec<PathBuf> = std::fs::read_dir(default_save_path(""))
                .map(|read_dir| {
                    read_dir
                        .filter_map(|entry| entry.ok().map(|e| e.path()))
                        .collect()
                })
                .unwrap_or_else(|_| Vec::new());

            if !account_files.is_empty() {
                let first_account = account_files[0].clone();
                load_from_file(Some(&first_account))
            } else {
                Some(Account::default())
            }
        };

        Self {
            project_root: project_root,
            contract_path: deploy_config.contract_path,
            network: network,
            deployer_args: deploy_config.deployer_args,
            account_name: deploy_config.account_name,
            account: deployment_account,
        }
    }

    pub fn deploy_contracts(
        &mut self,
        path_to_contracts: Option<PathBuf>,
    ) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
        println!("Deploying contracts...");
        let network: String = self.network.as_ref().unwrap().to_string();
        println!("Network: {:?}", network);
        let project_root = self.project_root.as_ref().unwrap().clone();
        let full_path_to_contracts: PathBuf = if path_to_contracts.is_some() {
            path_to_contracts.unwrap()
        } else {
            find_wasm_release_folder(&project_root)?
        };

        println!("Full path to contracts: {:?}", full_path_to_contracts);
        if !full_path_to_contracts.is_dir() {
            return Err("Path to contracts is not a directory".into());
        } else {
            println!("Path to contracts is a directory");
        }
        let contract_pbcs: Vec<PathBuf> = find_paths_with_extension(&full_path_to_contracts, "pbc");

        println!("Contract ABIs: {:?}", contract_pbcs);
        if contract_pbcs.is_empty() {
            return Err("No contracts found".into());
        }
        let mut results: Vec<Vec<u8>> = Vec::new();
        if &network == "testnet" {
            // check account has gas

            // if testnet and no gas, mint gas
            self.account.as_mut().unwrap().mint_gas();
            // deploy all contracts
            // find all contract abi's in target/wasm32-unknown-unknown/release

            // get contract name

            // deploy each contract

            for contract_pbc in contract_pbcs {
                let contract_name: &OsStr = contract_pbc.file_name().unwrap();
                println!("Contract Name: {:?}", contract_name);
                let contract_deploy_args: Vec<String> =
                    contract_deploy_args(&contract_name.to_str().unwrap())?;
                results.push(self.deploy_contract(
                    Some(contract_pbc),
                    None,
                    None,
                    None,
                    contract_deploy_args,
                )?);
            }
            return Ok(results);
        } else if &network == "mainnet" {
            for contract_pbc in contract_pbcs {
                let contract_name: &OsStr = contract_pbc.file_name().unwrap();
                let contract_deploy_args: Vec<String> =
                    contract_deploy_args(&contract_name.to_str().unwrap())?;
                self.deploy_contract(Some(contract_pbc), None, None, None, contract_deploy_args)?;
            }
            return Ok(results);
        } else {
            println!("Invalid network");
            return Err("Invalid network".into());
        }
    }

    pub fn deploy_contract(
        &mut self,
        contract_pbc_path: Option<PathBuf>,
        contract_abi_path: Option<PathBuf>,
        contract_wasm_path: Option<PathBuf>,
        contract_zkwa_path: Option<PathBuf>,
        args: Vec<String>,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // cargo partisia-contract transaction deploy --gas 10000000 --privatekey YourAccountFile.pk your_compiled_contract_file.pbc + contract inputs separated by spaces (strings in quotes)
        let account_name: String = self.account_name.as_ref().unwrap().to_string();
        let pk_file_path: PathBuf = default_save_path(&account_name);
        if !pk_file_path.is_file() {
            self.account
                .as_mut()
                .unwrap()
                .get_private_key(Some(&account_name))
                .expect("Failed to derive private key");
        }

        println!(
            "Command: cargo pbc transaction deploy --gas 10000000 --privatekey {:?} --abi{:?} {:?}",
            pk_file_path,
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
            .arg(pk_file_path);

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
            print_output(&deployment_tx);
            Ok(deployment_tx.stdout)
        } else {
            print_error(&deployment_tx);
            Err("Failed to deploy contract".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Debug, Clone)]
    struct DeployProjectTest {
        account_name: Option<String>,
    }
    #[test]
    fn test_deploy_contracts() {
        // create new project

        let deploy_config: DeployProject = DeployProject {
            account_name: Some("test".to_string()),
            project_root: find_workspace_root(),
            contract_path: None,
            network: None,
            deployer_args: None,
            account: None,
        };
        let mut deploy_project: DeployProject = DeployProject::new(deploy_config);
        let result = deploy_project
            .deploy_contracts(None)
            .expect("Failed to deploy contracts");
        println!("{:?}", result);
    }
}
