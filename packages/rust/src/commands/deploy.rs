// cargo pbc transaction deploy --net="testnet" target/wasm32-unknown-unknown/release/ballot.zkwa
// '[YES, NO]' "Does this work?" "Let's see..." --abi=target/wasm32-unknown-unknown/release/ballot.abi
// --pk ./Account-A.pk --gas=10000000

use std::path::PathBuf;
use std::env::current_dir;
pub struct DeployConfig {
    pub project_root: PathBuf,
    pub contract_path: PathBuf,
    pub network: String,
}

impl DeployConfig {
    pub fn new(deploy_config: DeployConfig) -> Self {
        let project_root = deploy_config.project_root.is_empty() ?             ? current_dir().unwrap()
            : deploy_config.project_root;
        Self {
            project_root: project_root,
            contract_path,
            network: String::from("testnet"), // default to testnet
        }
    }
}

pub fn execute(_config: DeployConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("⚠️  Deploy command not yet implemented");
    Ok(())
}
