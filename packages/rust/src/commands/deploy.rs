// cargo pbc transaction deploy --net="testnet" target/wasm32-unknown-unknown/release/ballot.zkwa
// '[YES, NO]' "Does this work?" "Let's see..." --abi=target/wasm32-unknown-unknown/release/ballot.abi
// --pk ./Account-A.pk --gas=10000000


use std::path::PathBuf;
use crate::{commands::account::{default_save_path, load_from_file, Account}, utils::utils::print_output};
use std::process::{Command, Output};
use std::ffi::OsStr;
use crate::utils::menus::contract_deploy_args;
use crate::utils::utils::find_paths_with_extension;
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
            let account_path = default_save_path(&deploy_config.account_name.as_ref().unwrap());
            if account_path.is_file() {
                load_from_file(Some(account_path))
            } else {
                Some(Account::new(
                    deploy_config.account_name.as_deref(),
                    network.as_deref(),
                    None, None, None, None
                ))
            }
        } else {
            // look in .account folder for first account
            let account_files: Vec<PathBuf> = std::fs::read_dir(default_save_path("")).unwrap()
                .map(|entry| entry.unwrap().path())
                .collect();
        
            if !account_files.is_empty() {
                let first_account = account_files[0].clone();
                load_from_file(Some(first_account))
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

    pub fn deploy_contracts(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Deploying contracts...");
        let network: String = self.network.as_ref().unwrap().to_string();
        let contract_abis: Vec<PathBuf> = find_paths_with_extension(&PathBuf::from(format!("{}/target/wasm32-unknown-unknown/release", self.project_root.as_ref().unwrap().display())), "pbc");
 
        if  &network == "testnet" {
            // check account has gas

            // if testnet and no gas, mint gas

            // deploy all contracts
            // find all contract abi's in target/wasm32-unknown-unknown/release
         
            // get contract name
          
            // deploy each contract
            for contract_abi in contract_abis {
                let contract_name: &OsStr = contract_abi.file_name().unwrap();
                let contract_deploy_args: Vec<String> = contract_deploy_args(&contract_name.to_str().unwrap())?;
                self.deploy_contract(contract_abi, contract_deploy_args)?;
            }
            

        } else if &network == "mainnet" {

            for contract_abi in contract_abis {
                let contract_name: &OsStr = contract_abi.file_name().unwrap();
                let contract_deploy_args: Vec<String> = contract_deploy_args(&contract_name.to_str().unwrap())?;
                self.deploy_contract(contract_abi, contract_deploy_args)?;
            }

        } else {
            println!("Invalid network");
            return Err("Invalid network".into());
        }
        Ok(())
    }

    pub fn deploy_contract(&mut self, contract_abi_path: PathBuf, args: Vec<String>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // cargo partisia-contract transaction deploy --gas 10000000 --privatekey YourAccountFile.pk your_compiled_contract_file.pbc + contract inputs separated by spaces (strings in quotes) 
        let deploymentTx: Output = Command::new("cargo")
        .arg("pbc")
        .arg("transaction")
        .arg("deploy")
        .arg("--gas")
        .arg("10000000")
        .arg("--privatekey") 
        .arg(self.account.as_mut().unwrap().derive_private_key().unwrap())
        .arg(contract_abi_path)
        .arg(args.join(" "))
        .output()
        .expect("Failed to show account");

        if deploymentTx.status.success() {
            print_output(&deploymentTx);
            Ok(deploymentTx.stdout)

        } else {
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
        let deploy_config: DeployProject = DeployProject::default();
        let mut deploy_project: DeployProject = DeployProject::new(deploy_config);
        let result = deploy_project.deploy_contracts().unwrap();
        println!("{:?}", result);
    }
}