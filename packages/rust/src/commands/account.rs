use crate::utils::constants::DEFAULT_NETWORK;
use crate::utils::fs_nav::{find_paths_with_name, find_workspace_root, get_pk_files, id_pbc_path};
use crate::utils::menus::{create_new_account_menu, force_new_wallet_menu, select_account_menu};
use crate::utils::utils::{
    create_pk_file, get_address_from_pk, load_account_from_pk_file, print_error, print_output,
    address_is_valid,
};

use serde::{Deserialize, Serialize};

use std::error::Error;

use std::path::PathBuf;
use std::process::{Command, Output};


pub struct AccountConfig {
    pub network: Option<String>,
    pub address: Option<String>,
    pub private_key: Option<String>,
    pub path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub network: String,
    pub address: String,
    pub private_key: String,
    pub path: PathBuf,
}

impl Default for Account {
    fn default() -> Self {
        let pk_files: Vec<PathBuf> = get_pk_files();
        if pk_files.len() > 0 {
            let account: Account = load_account_from_pk_file(&pk_files[0], DEFAULT_NETWORK).expect("Account::default: Failed to load account from file");
            return Self {
                network: DEFAULT_NETWORK.to_string(),
                address: account.address,
                private_key: account.private_key,
                path: pk_files[0].clone(),
            };
        } else {
            if id_pbc_path().is_none() {
                println!("no wallet, creating new one");
                // if there is no wallet, create a new one
                pbc_create_new_wallet(DEFAULT_NETWORK).expect("Default account: Failed to create new wallet");
            }
            println!("creating new account");
            // create new account
            pbc_create_new_account(DEFAULT_NETWORK).expect("Default account: Failed to create new account");

            let pk_files: Vec<PathBuf> = get_pk_files();
            let path: PathBuf = if pk_files.len() > 0 {
                pk_files[0].clone()
            } else {
                panic!("Default Account is failing to create a new account");
            };
            let default_account: Account =
                load_account_from_pk_file(&path, DEFAULT_NETWORK).expect("Default account: Failed to load account from file");
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
        account_config: AccountConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // initialize new account
        let new_account: Self;
        let network: String = account_config.network.unwrap_or(DEFAULT_NETWORK.to_string());
        // if path to pk is provided, load account from file
        if account_config.path.is_some() {
            let path_is_file: bool = account_config.path.as_ref().unwrap().is_file();
            let path: PathBuf = account_config.path.as_ref().unwrap().clone();
            if !path_is_file {
                return Err(format!("Account::new: Path to private key is not a file: {}", &path.display()).into());
            }
            println!("loading account from file: {}, {}", &path.display(), &network);
            new_account = load_account_from_pk_file(&path, &network).expect("Account::new: Failed to load account from file");

            return Ok(Self{
                network: network,
                private_key: new_account.private_key,
                address: new_account.address,
                path: path,
            });
        }            

        match (account_config.address, account_config.private_key) {
            (Some(address), Some(private_key)) => {
                // validate address and private key
                let is_valid: bool = address_is_valid(&address, &private_key).unwrap_or(false);
                if !is_valid {
                    return Err(format!(
                        "Invalid address: {} or private key: {}",
                        address, private_key
                    )
                    .into());
                }
                let path:PathBuf  = create_pk_file(&private_key)?;

                new_account = Self {
                    network: network,
                    private_key: private_key,
                    address: address,
                    path: path,
                };
            }
            (None, Some(private_key)) => {
                let final_address: String = get_address_from_pk(&private_key)?;
                let path: PathBuf = create_pk_file(&private_key)?;

                let is_valid: bool = address_is_valid(&final_address, &private_key).unwrap_or(false);
                if !is_valid {
                    return Err(format!(
                        "Invalid address: {} or private key: {}",
                        &final_address, &private_key
                    )
                    .into());
                }
                new_account = Self {
                    network: network,
                    private_key: private_key,
                    address: final_address,
                    path: path,
                };
            }
            (Some(address), None) => {
                let final_address: String = address.to_string();
                // look for pk file with address in name
                let pk_files: Vec<PathBuf> = find_paths_with_name(
                    &find_workspace_root().unwrap(),
                    &final_address,
                );


                match pk_files.len() {
                    1 => {
                        new_account = load_account_from_pk_file(&pk_files[0], &network).unwrap_or_else(|e| {
                            panic!("Account::new: Failed to load account from file: {}", e);
                        });
                    }
                    n if n > 1 => {
                        let account_file: PathBuf = select_account_menu().unwrap();
                        new_account = load_account_from_pk_file(&account_file, &network).unwrap();
                    }
                    0 => {
                        new_account = create_new_account_menu().unwrap();
                    },
                    _ => {
                        return Err(
                            "the number of pk files is not 0, 1, or greater than 1.  This is inconceivable".into(),
                        );
                    }
                }
            }
            (None, None) => {
                let pk_files: Vec<PathBuf> = get_pk_files();
                let pk_files_len: usize = pk_files.len();
                match pk_files_len {
                    1 => {
                        new_account = load_account_from_pk_file(&pk_files[0], &network).unwrap();
                    }
                    n if n > 1 => {
                        let account_file: PathBuf = select_account_menu().unwrap();
                        new_account = load_account_from_pk_file(&account_file, &network).unwrap();
                    }
                    0 => {
                        new_account = create_new_account_menu().unwrap();
                    },
                    _ => {
                        return Err(
                            "the number of pk files is not 0, 1, or greater than 1.  This is inconceivable".into(),
                        );
                    }
                }
            }

        }
        Ok(new_account)
    }

    #[allow(dead_code)]
    pub fn load_account_from_path(
        &mut self,
        network: Option<&str>,
        path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("loading account from path: {}, {}", &path.display(), &network.unwrap_or(&self.network));
        let account =
            load_account_from_pk_file(path, network.unwrap_or(&self.network));
        if account.is_err() {
            return Err(format!("Failed to load account from path: {}", account.err().unwrap()).into());
        }
        let account = account.unwrap();
        self.network = account.network;
        self.address = account.address;
        self.private_key = account.private_key;
        self.path = path.clone();
        Ok(())
    }

    #[allow(dead_code)]
    pub fn update_private_key(
        &mut self,
        private_key: &str,
        network: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let address: String = get_address_from_pk(private_key).unwrap_or_else(|e| {
            panic!("Account::update_private_key: Failed to get address from pk: {}", e);
        });
        self.network = network.unwrap_or(&self.network).to_string();
        self.address = address;
        self.private_key = private_key.to_string();
        Ok(())
    }

    #[allow(dead_code)]
    pub fn update_address(&mut self, address: &str) {
        self.address = address.to_string();
    }

    #[allow(dead_code)]
    pub fn update_network(&mut self, network: &str) {
        self.network = network.to_string();
    }

    pub fn mint_gas(&self) -> Result<(), Box<dyn std::error::Error>> {
        // can only mint gas on testnet
        let network_command: String = format!("--net=testnet");

        let output = Command::new("cargo")
            .arg("pbc")
            .arg("account")
            .arg("mintgas")
            .arg(network_command)
            .arg(&self.address)
            .output()
            .expect("Failed to mint gas");
        if output.status.success() {
            return print_output("mint_gas", &output);
        } else {
            return print_error(&output);
        }
    }

    #[allow(dead_code)]
    pub fn private_key(&self) -> String {
        self.private_key.clone()
    }

    #[allow(dead_code)]
    pub fn address(&self) -> String {
        self.address.clone()
    }

    pub fn show_account(
        &self
    ) -> Result<String, Box<dyn Error + 'static>> {
        let network_command = format!("--net={}", &self.network);
            let shown_account: Output = Command::new("cargo")
                .arg("pbc")
                .arg("account")
                .arg("show")
                .arg(network_command)
                .arg(&self.address)
                .output()
                .expect("Failed to show account");

            if shown_account.status.success() {
                return print_output("show_account", &shown_account);
            } else {
                return print_error(&shown_account);
            }

    }
}

pub fn pbc_create_new_account(network: &str) -> Result<Account, Box<dyn std::error::Error>> {
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
        return print_output("pbc_create_new_account", &output);
    }
}

pub fn pbc_create_new_wallet(network: &str) -> Result<String, Box<dyn std::error::Error>> {
    let network_command: String = format!("--net={}", network);
    // check if .pbc folder exists
    if id_pbc_path().is_none() {
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
            return print_output("pbc_create_new_wallet no force", &output);
        }
    } else if id_pbc_path().is_some() {
        // open menu to ask if user wants to create a new account
        let force_create: bool = force_new_wallet_menu().expect("Failed force new wallet menu");
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
                return print_output("pbc_create_new_wallet force", &output);
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

    fn setup_default_account() -> Account {
        // create a new account
        let account: Account = Account::default();
        assert_eq!(account.clone().private_key().len() > 0, true, "private key is not set");
        assert_eq!(account.clone().address().len() > 0, true, "address is not set");
        assert_eq!(account.clone().network, "testnet", "network is not set");
        return account;
    }

    fn setup_account_from_path(path: &PathBuf) -> Account {
            let account: Account = Account::new(AccountConfig {
                network: Some("mainnet".to_string()),
                address: None,
                private_key: None,
                path: Some(path.clone()),
            }).unwrap();
            assert_eq!(account.clone().private_key().len() > 0, true, "private key is not set");
            assert_eq!(account.clone().address().len() > 0, true, "address is not set");
            assert_eq!(account.clone().network, "mainnet", "network is not set");
            return account;
        
    }

    #[test]
    fn test_create_new_default_account() {
        let account: Account = setup_default_account();
        assert_eq!(account.clone().private_key().len() > 0, true, "private key is not set");
        assert_eq!(account.clone().address().len() > 0, true, "address is not set");
        assert_eq!(account.clone().network, "testnet", "network is not set");
    }

    #[test]
    fn test_load_account_from_path_invalid_path() {
        let mut account: Account = Account::default();
        let result = account.load_account_from_path(Some("testnet"), &PathBuf::from("invalid_path"));
        assert_eq!(result.is_err(), true, "should be an error");
        assert_eq!(result.err().unwrap().to_string(), "Failed to load account from path: load_account_from_pk_file: Invalid path: invalid_path");
    }

    #[test]
    fn test_create_new_account_from_path() {
        // get file path to pk file
        let pk_files: Vec<PathBuf> = get_pk_files();
        if pk_files.len() > 0 {
            let account: Account = setup_account_from_path(&pk_files[0]);
            assert_eq!(account.clone().private_key().len() > 0, true, "private key is not set");
            assert_eq!(account.clone().address().len() > 0, true, "address is not set");
            assert_eq!(account.clone().network, "mainnet", "network is not set");
        } else {
            println!("no pk files found");
        }


    }

    #[test]
    fn test_load_account_from_path() {
        // find a pk file in root
        let pk_files: Vec<PathBuf> = get_pk_files();
        assert_eq!(pk_files.len() > 0, true, "no pk files found");
        if pk_files.len() > 0 {
        let mut account: Account = Account::default();
        account.load_account_from_path(Some("testnet"), &pk_files[0]).unwrap();
        assert_eq!(account.clone().private_key().len() > 0, true, "private key is not set");
        assert_eq!(account.clone().address().len() > 0, true, "address is not set");
        assert_eq!(account.clone().network, "testnet", "network is not set");
        assert_eq!(account.clone().path.is_file(), true, "path is not a file");
        assert_eq!(account.clone().path.is_dir(), false, "path is not a file");
        } else {
            println!("must create a new account");
        }
    }
}
