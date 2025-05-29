// cargo pbc transaction deploy --net="testnet" target/wasm32-unknown-unknown/release/ballot.zkwa
// '[YES, NO]' "Does this work?" "Let's see..." --abi=target/wasm32-unknown-unknown/release/ballot.abi
// --pk ./Account-A.pk --gas=10000000

use std::path::PathBuf;
use std::env::current_dir;
use std::process::Command;

use pbc_contract_common::address::AddressType;

#[derive(Debug, Clone)]
pub struct DeployProject {
    pub project_root: Option<PathBuf>,
    pub contract_path: Option<PathBuf>,
    pub network: Option<String>,       
    pub deployer_args: Option<Vec<String>>,
}

impl DeployProject {
    pub fn new(deploy_config: DeployProject) -> Self {
        Self {
            project_root: deploy_config.project_root,
            contract_path: deploy_config.contract_path,
            network: deploy_config.network,
            deployer_args: deploy_config.deployer_args,
        }
    }

    pub fn deploy_contracts(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Deploying contracts...");
        // check env for private key
        let private_key: String = std::env::var("PRIVATE_KEY").unwrap_or(String::from(""));
        let network: String = self.network.as_ref().unwrap().to_string();
        // if no private key create a new test account and write private key to env
        if  &network == "testnet" {
            // let new_account: AddressType = create_account();
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
        create_account();
        // let project_root = if self.project_root.is_none() {
        //     current_dir().unwrap()
        // } else {
        //     self.project_root.unwrap()
        // };
        Ok(())
    }
}

pub fn create_account() {
    let account = Command::new("cargo")
        .arg("pbc")
        .arg("wallet")
        .arg("create")
        .output()
        .expect("Failed to create account");
    println!("Account created: {:#?}", account);
    let account_str = String::from_utf8(account.stdout).unwrap();
    // let account: AddressType = account_str.parse().unwrap();
    // account
}

