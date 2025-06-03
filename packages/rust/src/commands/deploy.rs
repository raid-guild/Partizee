// cargo pbc transaction deploy --net="testnet" target/wasm32-unknown-unknown/release/ballot.zkwa
// '[YES, NO]' "Does this work?" "Let's see..." --abi=target/wasm32-unknown-unknown/release/ballot.abi
// --pk ./Account-A.pk --gas=10000000


use std::path::PathBuf;
use crate::commands::account::{Account, load_from_file, default_save_path};
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

    pub fn deploy_contracts(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Deploying contracts...");
        let network: String = self.network.as_ref().unwrap().to_string();
        if  &network == "testnet" {
            let account: Account = self.account.as_ref().unwrap().clone();
            // check account has gas

        } else if &network == "mainnet" {

        } else {
            println!("Invalid network");
            return Err("Invalid network".into());
        }

        // if private key exists, check for gas

        // if gas exists, deploy contracts

        // if gas doesn't exist mint gas

        // deploy contracts
        // let project_root = if self.project_root.is_none() {
        //     current_dir().unwrap()
        // } else {
        //     self.project_root.unwrap()
        // };
        Ok(())
    }

    pub fn deployment_tx(&self) {
        //cargo partisia-contract transaction deploy --gas 10000000 --privatekey YourAccountFile.pk your_compiled_contract_file.pbc + contract inputs separated by spaces (strings in quotes) 
        // let deploymentTx: Output = Command::new("cargo")
        // .arg("pbc")
        // .arg("transaction")
        // .arg("deploy")
        // .arg("--gas")
        // .arg("10000000")
        // .arg("--privatekey")
        // .arg(self.account.as_ref().unwrap().pk_path.as_ref().unwrap())
        // .arg(self.contract_path.as_ref().unwrap())
        // .output()
        // .expect("Failed to show account");

        // if deploymentTx.status.success() {
        //     Ok(deploymentTx)
        // } else {
        //     Err("Failed to deploy contract".into())
        // }
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
        let deploy_config: DeployProject = DeployProject {
            account_name: Some("test".to_string()),
            project_root: None,
            contract_path: None,
            network: None,
            deployer_args: None,
            account: None,
        };
        let deploy_project: DeployProject = DeployProject::new(deploy_config);
        deploy_project.deploy_contracts().unwrap();
    }
}