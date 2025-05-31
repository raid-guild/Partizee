// cargo pbc transaction deploy --net="testnet" target/wasm32-unknown-unknown/release/ballot.zkwa
// '[YES, NO]' "Does this work?" "Let's see..." --abi=target/wasm32-unknown-unknown/release/ballot.abi
// --pk ./Account-A.pk --gas=10000000


use std::path::PathBuf;
use crate::commands::account::Account;

#[derive(Debug, Clone)]
pub struct DeployProject {
    pub project_root: Option<PathBuf>,
    pub contract_path: Option<PathBuf>,
    pub network: Option<String>,       
    pub deployer_args: Option<Vec<String>>,
    pub account: Option<Account>,
}

impl DeployProject {
    pub fn new(deploy_config: DeployProject) -> Self {
        let account: Option<Account> = Some(Account::new(deploy_config.network.as_deref()));
        Self {
            project_root: deploy_config.project_root,
            contract_path: deploy_config.contract_path,
            network: deploy_config.network,
            deployer_args: deploy_config.deployer_args,
            account: account,
        }
    }

    pub fn deploy_contracts(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Deploying contracts...");
        let network: String = self.network.as_ref().unwrap().to_string();
        // if no private key create a new test account and write private key to env
        if  &network == "testnet" {
            println!("Creating testnet account...");
            // std::env::set_var("PRIVATE_KEY", new_account);
        } else if &network == "mainnet" {
            // let new_account: AddressType = create_account();
            // std::env::set_var("PRIVATE_KEY", new_account);
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
}


