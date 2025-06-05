use crate::utils::constants::DEFAULT_NETWORK;
use crate::utils::menus::{create_new_account_menu, force_new_wallet_menu, select_account_menu};
use crate::utils::utils::{
    create_pk_file, find_paths_with_name, get_address_from_pk, get_pk_files, id_pbc_path,
    load_account_from_pk_file, print_error, print_output, validate_address,
};

use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub network: String,
    pub address: Option<String>,
    pub private_key: Option<String>,
    pub path: PathBuf,
}

impl Default for Account {
    fn default() -> Self {
        let pk_files: Vec<PathBuf> = get_pk_files();
        println!("pk_files: {:?}", pk_files);
        if !pk_files.is_empty() {
            let account: Account = load_account_from_pk_file(&pk_files[0], "testnet").unwrap();
            return Self {
                network: DEFAULT_NETWORK.to_string(),
                address: account.address,
                private_key: account.private_key,
                path: pk_files[0].clone(),
            };
        } else {
            if !id_pbc_path().unwrap().is_file() {
                println!("no wallet, creating new one");
                // if there is no wallet, create a new one
                create_new_wallet(DEFAULT_NETWORK).expect("Failed to create new wallet");
            }
            println!("creating new account");
            // create new account
            create_new_account(DEFAULT_NETWORK).expect("Failed to create new account");

            let pk_files: Vec<PathBuf> = get_pk_files();
            let path: PathBuf = pk_files[0].clone();
            let default_account: Account =
                load_account_from_pk_file(&path, DEFAULT_NETWORK).unwrap();
            return Self {
                network: default_account.network,
                address: default_account.address,
                private_key: default_account.private_key,
                path: path,
            };
        }
    }
}

impl Account {
    pub fn new(
        path_to_pk: Option<&PathBuf>,
        network: Option<&str>,
        address: Option<&str>,
        private_key: Option<&str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // initialize new account
        let new_account: Self;
        let network: String = network.unwrap_or(DEFAULT_NETWORK).to_string();
        let mut final_address: String;
        let final_private_key: String;
        let (address_opt, private_key_opt) = (address, private_key);
        // if path to pk is provided, load account from file
        if path_to_pk.is_some() {
            println!("loading account from file");
            new_account = load_account_from_pk_file(path_to_pk.unwrap(), &network)
                .expect("Failed to load account from file");
            return Ok(new_account);
        }

        match (address_opt, private_key_opt) {
            (Some(address), Some(private_key)) => {
                // validate address and private key
                if !validate_address(address, private_key).unwrap() {
                    return Err(format!(
                        "Invalid address: {} or private key: {}",
                        address, private_key
                    )
                    .into());
                }
                final_address = address.to_string();
                final_private_key = private_key.to_string();
                new_account = Self {
                    network: network,
                    private_key: Some(final_private_key),
                    address: Some(final_address),
                    path: PathBuf::from(""),
                };
            }
            (None, Some(private_key)) => {
                final_private_key = private_key.to_string();
                final_address = get_address_from_pk(private_key).unwrap();
                let path: PathBuf = create_pk_file(&final_private_key).unwrap();
                if !validate_address(&final_address, &final_private_key).unwrap() {
                    return Err(format!(
                        "Invalid address: {} or private key: {}",
                        &final_address, &final_private_key
                    )
                    .into());
                }
                new_account = Self {
                    network: network,
                    private_key: Some(final_private_key),
                    address: Some(final_address),
                    path: path,
                };
            }
            (Some(address), None) => {
                final_address = address.to_string();
                // look for pk file with address in name
                let pk_files: Vec<PathBuf> = find_paths_with_name(
                    &PathBuf::from(env!("CARGO_MANIFEST_DIR")),
                    &final_address,
                );

                if pk_files.len() > 0 {
                    final_private_key = fs::read_to_string(pk_files[0].clone()).unwrap();
                    final_address = address.to_string();
                    new_account = Self {
                        network: network,
                        private_key: Some(final_private_key),
                        address: Some(final_address),
                        path: pk_files[0].clone(),
                    };
                } else {
                    return Err(format!("No account file found for address: {}", &address).into());
                }
            }
            (None, None) => {
                let pk_files: Vec<PathBuf> = get_pk_files();
                match pk_files.len() {
                    1 => {
                        new_account = load_account_from_pk_file(&pk_files[0], &network).unwrap();
                    }
                    n if n > 1 => {
                        let account_file: PathBuf = select_account_menu().unwrap();
                        new_account = load_account_from_pk_file(&account_file, &network).unwrap();
                    }
                    0 => {
                        new_account = create_new_account_menu().unwrap();
                    }
                    _ => {
                        return Err(
                            "you have made it to limbo.  look around, enjoy yourself".into()
                        );
                    }
                }
            }
            _ => {
                return Err(
                    "Something has gone wrong.  Please report this issue to the developers".into(),
                );
            }
        }
        Ok(new_account)
    }

    pub fn load_account_from_path(
        &mut self,
        network: Option<&str>,
        path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let account: Account =
            load_account_from_pk_file(path, network.unwrap_or(&self.network)).unwrap();
        self.network = account.network;
        self.address = account.address;
        self.private_key = account.private_key;
        Ok(())
    }

    pub fn update_private_key(
        &mut self,
        private_key: &str,
        network: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let address: String = get_address_from_pk(private_key).unwrap();
        self.network = network.unwrap_or(&self.network).to_string();
        self.address = Some(address);
        self.private_key = Some(private_key.to_string());
        Ok(())
    }

    pub fn update_address(&mut self, address: &str) {
        self.address = Some(address.to_string());
    }
    pub fn update_network(&mut self, network: &str) {
        self.network = network.to_string();
    }

    pub fn mint_gas(&self) -> Result<(), Box<dyn std::error::Error>> {
        // can only mint gas on testnet
        let network_command: String = format!("--net=testnet");
        // check if address is not None
        if self.address.is_none() {
            return Err("Account address is not set".into());
        }
        let output = Command::new("cargo")
            .arg("pbc")
            .arg("account")
            .arg("mintgas")
            .arg(network_command)
            .arg(self.address.as_ref().unwrap())
            .output()
            .expect("Failed to mint gas");
        if output.status.success() {
            return print_output("mint_gas", &output);
        } else {
            return print_error(&output);
        }
    }

    pub fn private_key(&self) -> Option<String> {
        self.private_key.clone()
    }

    pub fn address(&self) -> Option<String> {
        self.address.clone()
    }

    pub fn show_account(
        &self,
        network: Option<&str>,
        address: Option<&str>,
    ) -> Result<String, Box<dyn Error + 'static>> {
        let network_command = format!("--net={}", network.unwrap_or(&self.network));
        let address: String = address
            .unwrap_or(&self.address.clone().unwrap_or("".to_string()))
            .to_string();
        if address.is_empty() {
            return Err("Account address is not set".into());
        } else {
            let shown_account: Output = Command::new("cargo")
                .arg("pbc")
                .arg("account")
                .arg("show")
                .arg(network_command)
                .arg(&address)
                .output()
                .expect("Failed to show account");

            if shown_account.status.success() {
                return print_output("show_account", &shown_account);
            } else {
                return print_error(&shown_account);
            }
        }
    }
}

pub fn create_new_account(network: &str) -> Result<Account, Box<dyn std::error::Error>> {
    let network_command: String = format!("--net={}", &network);
    let output = Command::new("cargo")
        .arg("pbc")
        .arg("account")
        .arg("create")
        .arg(&network_command)
        .output()
        .expect("Failed to create account");
    if !output.status.success() {
        return print_error(&output);
    } else {
        return print_output("create_new_account", &output);
    }
}

pub fn create_new_wallet(network: &str) -> Result<String, Box<dyn std::error::Error>> {
    let network_command: String = format!("--net={}", network);
    // check if .pbc folder exists
    if !id_pbc_path().unwrap().is_file() {
        // create new account
        let output = Command::new("cargo")
            .arg("pbc")
            .arg("wallet")
            .arg("create")
            .arg(&network_command)
            .output()
            .expect("Failed to create account");

        if !output.status.success() {
            return print_error(&output);
        } else {
            return print_output("create_new_wallet no force", &output);
        }
    } else if id_pbc_path().unwrap().is_file() {
        // open menu to ask if user wants to create a new account
        let force_create: bool = force_new_wallet_menu().unwrap();
        if force_create {
            let output = Command::new("cargo")
                .arg("pbc")
                .arg("wallet")
                .arg("create")
                .arg(&network_command)
                .arg("--force")
                .output()
                .expect("Failed to create account");

            if !output.status.success() {
                return print_error(&output);
            } else {
                return print_output("create_new_wallet force", &output);
            }
        } else {
            return Err("Failed to create wallet".into());
        }
    } else {
        return Err("I didn't expect to get here.  I mean, the id_pbc file exists or it doesnt.  There shouldn't be a third option.".into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::utils::get_account_address_from_path;
    use std::sync::LazyLock;

    #[test]
    fn test_load_from_file() {
        // find a pk file in root
        let pk_files: Vec<PathBuf> = get_pk_files();
        if !pk_files.is_empty() {
            let loaded_account: Account =
                load_account_from_pk_file(&pk_files[0], "testnet").unwrap();
            assert!(loaded_account.private_key().is_some());
        }
    }

    #[test]
    fn test_get_private_key() {
        let pk_files: Vec<PathBuf> = get_pk_files();
        if !pk_files.is_empty() {
            let account: Account =
                Account::new(Some(&pk_files[0]), Some("testnet"), None, None).unwrap();
            let private_key: String = account.private_key().unwrap();
            assert!(private_key.len() == 64);
        }
    }
}
