// cargo pbc transaction deploy --net="testnet" target/wasm32-unknown-unknown/release/ballot.zkwa
// '[YES, NO]' "Does this work?" "Let's see..." --abi=target/wasm32-unknown-unknown/release/ballot.abi
// --pk ./Account-A.pk --gas=10000000

use std::path::PathBuf;

pub struct DeployConfig {
    pub contract_path: PathBuf,
    pub network: String,
}

impl DeployConfig {
    pub fn new(contract_path: PathBuf) -> Self {
        Self {
            contract_path,
            network: String::from("testnet"), // default to testnet
        }
    }
}

pub fn execute(_config: DeployConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("⚠️  Deploy command not yet implemented");
    Ok(())
}
