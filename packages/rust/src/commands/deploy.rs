// cargo pbc transaction deploy --net="testnet" target/wasm32-unknown-unknown/release/ballot.zkwa
// '[YES, NO]' "Does this work?" "Let's see..." --abi=target/wasm32-unknown-unknown/release/ballot.abi
// --pk ./Account-A.pk --gas=10000000

use std::path::PathBuf;
use std::env::current_dir;
pub struct DeployProject {
    pub project_root: Option<PathBuf>,
    pub contract_path: Option<PathBuf>,
    pub network: Option<String>,       
    pub deployer_args: Option<Vec<String>>,
}

impl DeployProject {
    pub fn new(deploy_config: DeployProject) -> Self {
        let project_root = if deploy_config.project_root.is_none() {
            current_dir().unwrap()
        } else {
            deploy_config.project_root.unwrap()
        };

        let contract_path = if deploy_config.contract_path.is_none() {
            PathBuf::from("target/wasm32-unknown-unknown/release/")
        } else {
            deploy_config.contract_path.unwrap()
        };

        let network = if deploy_config.network.is_none() {
            String::from("testnet")
        } else {
            deploy_config.network.unwrap()
        };

        Self {
            project_root: Some(project_root),
            contract_path: Some(contract_path),
            network: Some(network),
            deployer_args: deploy_config.deployer_args,
        }
    }

    pub fn deploy_contracts(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Deploying contract...");
        Ok(())
    }
}

