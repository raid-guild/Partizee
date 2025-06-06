use crate::utils::constants::DEFAULT_NETWORK;
use crate::utils::fs_nav::{find_paths_with_name, find_workspace_root, get_pk_files, id_pbc_path};
use crate::utils::menus::{create_new_profile_menu, select_pk_menu};
use crate::utils::pbc_commands::{pbc_create_new_account, pbc_create_new_wallet};
use crate::utils::utils::{
    address_is_valid, create_pk_file, get_address_from_pk, load_account_from_pk_file, print_error,
    print_output,
};
use serde::{Deserialize, Serialize};
use std::env;

use std::error::Error;

use std::path::PathBuf;
use std::process::{Command, Output};

pub struct ProfileConfig {
    pub network: Option<String>,
    pub address: Option<String>,
    pub private_key: Option<String>,
    pub path_to_pk: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub network: String,
    pub address: String,
    pub private_key: String,
    pub path_to_pk: PathBuf,
}

#[allow(dead_code)]
impl Default for Profile {
    fn default() -> Self {
        let pk_files: Vec<PathBuf> = get_pk_files();
        if pk_files.len() > 0 {
            let account: Profile = load_account_from_pk_file(&pk_files[0], DEFAULT_NETWORK)
                .unwrap_or_else(|e| {
                    panic!("Default account: Failed to load account from file: {}", e);
                });

            return Self {
                network: DEFAULT_NETWORK.to_string(),
                address: account.address,
                private_key: account.private_key,
                path_to_pk: pk_files[0].clone(),
            };
        } else {
            if id_pbc_path().is_none() {
                println!("no wallet, creating new one");
                // if there is no wallet, create a new one
                let result = pbc_create_new_wallet(DEFAULT_NETWORK);
                if result.is_err() {
                    println!(
                        "Default account: Failed to create new wallet: {}",
                        result.err().unwrap()
                    );
                }
            }
            println!("creating new account");
            // create new account
            pbc_create_new_account(DEFAULT_NETWORK)
                .expect("Default account: Failed to create new account");

            let pk_files: Vec<PathBuf> = get_pk_files();
            let path_to_pk: PathBuf = if pk_files.len() > 0 {
                pk_files[0].clone()
            } else {
                panic!("Default Profile is failing to create a new account");
            };
            let default_account: Profile = load_account_from_pk_file(&path_to_pk, DEFAULT_NETWORK)
                .expect("Default account: Failed to load account from file");
            return Self {
                network: default_account.network,
                address: default_account.address,
                private_key: default_account.private_key,
                path_to_pk: path_to_pk,
            };
        }
    }
}

#[allow(dead_code)]
impl Profile {
    pub fn new(account_config: ProfileConfig) -> Result<Self, Box<dyn std::error::Error>> {
        // initialize new account
        let new_profile: Self;
        let network: String = account_config
            .network
            .unwrap_or(DEFAULT_NETWORK.to_string());
        // if path_to_pk to pk is provided, load account from file
        if account_config.path_to_pk.is_some() {
            let path_to_pk_is_file: bool = account_config.path_to_pk.as_ref().unwrap().is_file();
            let path_to_pk: PathBuf = account_config.path_to_pk.as_ref().unwrap().clone();
            if !path_to_pk_is_file {
                return Err(format!(
                    "Profile::new: Path to private key is not a file: {}",
                    &path_to_pk.display()
                )
                .into());
            }
            println!(
                "loading account from file: {}, {}",
                &path_to_pk.display(),
                &network
            );
            new_profile = load_account_from_pk_file(&path_to_pk, &network)
                .expect("Profile::new: Failed to load account from file");

            return Ok(Self {
                network: network,
                private_key: new_profile.private_key,
                address: new_profile.address,
                path_to_pk: path_to_pk,
            });
        }

        match (account_config.address, account_config.private_key) {
            (Some(address), Some(private_key)) => {
                // validate address and private key
                let is_valid: bool = address_is_valid(&address, &private_key)?;
                if !is_valid {
                    return Err(format!(
                        "Invalid address: {} or private key: {}",
                        address, private_key
                    )
                    .into());
                }
                let path_to_pk: PathBuf = create_pk_file(&private_key)?;

                new_profile = Self {
                    network: network,
                    private_key: private_key,
                    address: address,
                    path_to_pk: path_to_pk,
                };
            }
            (None, Some(private_key)) => {
                let final_address: String = get_address_from_pk(&private_key)?;
                let path_to_pk: PathBuf = create_pk_file(&private_key)?;

                let is_valid: bool = address_is_valid(&final_address, &private_key)?;
                if !is_valid {
                    return Err(format!(
                        "Invalid address: {} or private key: {}",
                        &final_address, &private_key
                    )
                    .into());
                }
                new_profile = Self {
                    network: network,
                    private_key: private_key,
                    address: final_address,
                    path_to_pk: path_to_pk,
                };
            }
            (Some(address), None) => {
                let final_address: String = address.to_string();
                // look for pk file with address in name
                let pk_files: Vec<PathBuf> = find_paths_with_name(
                    &find_workspace_root().unwrap_or(env::current_dir().unwrap()),
                    &final_address,
                );

                match pk_files.len() {
                    1 => {
                        new_profile = load_account_from_pk_file(&pk_files[0], &network)
                            .unwrap_or_else(|e| {
                                panic!("Profile::new: Failed to load account from file: {}", e);
                            });
                    }
                    n if n > 1 => {
                        let account_file: PathBuf = select_pk_menu()?;
                        new_profile = load_account_from_pk_file(&account_file, &network)?;
                    }
                    0 => {
                        new_profile = create_new_profile_menu()?;
                    }
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
                        new_profile = load_account_from_pk_file(&pk_files[0], &network)?;
                    }
                    n if n > 1 => {
                        let account_file: PathBuf = select_pk_menu()?;
                        new_profile = load_account_from_pk_file(&account_file, &network)?;
                    }
                    0 => {
                        new_profile = create_new_profile_menu()?;
                    }
                    _ => {
                        return Err(
                            "the number of pk files is not 0, 1, or greater than 1.  This is inconceivable".into(),
                        );
                    }
                }
            }
        }
        Ok(new_profile)
    }

    pub fn load_account_from_path_to_pk(
        &mut self,
        network: Option<&str>,
        path_to_pk: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let account = load_account_from_pk_file(path_to_pk, network.unwrap_or(&self.network));
        if account.is_err() {
            return Err(format!(
                "Failed to load account from path_to_pk: {}",
                account.err().unwrap()
            )
            .into());
        }
        let account = account.unwrap();
        self.network = account.network;
        self.address = account.address;
        self.private_key = account.private_key;
        self.path_to_pk = path_to_pk.clone();
        Ok(())
    }

    pub fn update_private_key(
        &mut self,
        private_key: &str,
        network: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let address: String = get_address_from_pk(private_key).unwrap_or_else(|e| {
            panic!(
                "Profile::update_private_key: Failed to get address from pk: {}",
                e
            );
        });
        self.network = network.unwrap_or(&self.network).to_string();
        self.address = address;
        self.private_key = private_key.to_string();
        Ok(())
    }

    pub fn update_address(&mut self, address: &str) {
        self.address = address.to_string();
    }

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

    pub fn private_key(&self) -> String {
        self.private_key.clone()
    }

    pub fn address(&self) -> String {
        self.address.clone()
    }

    pub fn show_account(&self) -> Result<String, Box<dyn Error + 'static>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_default_account() -> Profile {
        // create a new account
        let account: Profile = Profile::default();
        assert_eq!(
            account.clone().private_key().len() > 0,
            true,
            "private key is not set"
        );
        assert_eq!(
            account.clone().address().len() > 0,
            true,
            "address is not set"
        );
        assert_eq!(account.clone().network, "testnet", "network is not set");
        return account;
    }

    fn setup_account_from_path(path: &PathBuf) -> Profile {
        let account: Profile = Profile::new(ProfileConfig {
            network: Some("mainnet".to_string()),
            address: None,
            private_key: None,
            path_to_pk: Some(path.clone()),
        })
        .unwrap();
        assert_eq!(
            account.clone().private_key().len() > 0,
            true,
            "private key is not set"
        );
        assert_eq!(
            account.clone().address().len() > 0,
            true,
            "address is not set"
        );
        assert_eq!(account.clone().network, "mainnet", "network is not set");
        return account;
    }

    #[test]
    fn test_create_new_default_account() {
        let account: Profile = setup_default_account();
        assert_eq!(
            account.clone().private_key().len() > 0,
            true,
            "private key is not set"
        );
        assert_eq!(
            account.clone().address().len() > 0,
            true,
            "address is not set"
        );
        assert_eq!(account.clone().network, "testnet", "network is not set");
    }

    #[test]
    fn test_load_account_from_path_to_pk_invalid_path() {
        let mut account: Profile = Profile::default();
        let result =
            account.load_account_from_path_to_pk(Some("testnet"), &PathBuf::from("invalid_path"));
        assert_eq!(result.is_err(), true, "should be an error");
        assert_eq!(result.err().unwrap().to_string(), "Failed to load account from path_to_pk: load_account_from_pk_file: Failed to read file: invalid_path");
    }

    #[test]
    fn test_create_new_account_from_path() {
        // get file path to pk file
        let pk_files: Vec<PathBuf> = get_pk_files();
        if pk_files.len() > 0 {
            let account: Profile = setup_account_from_path(&pk_files[0]);
            assert_eq!(
                account.clone().private_key().len() > 0,
                true,
                "private key is not set"
            );
            assert_eq!(
                account.clone().address().len() > 0,
                true,
                "address is not set"
            );
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
            let mut account: Profile = Profile::default();
            account
                .load_account_from_path_to_pk(Some("testnet"), &pk_files[0])
                .unwrap();
            assert_eq!(
                account.clone().private_key().len() > 0,
                true,
                "private key is not set"
            );
            assert_eq!(
                account.clone().address().len() > 0,
                true,
                "address is not set"
            );
            assert_eq!(account.clone().network, "testnet", "network is not set");
            assert_eq!(account.clone().path_to_pk.is_file(), true, "path is not a file");
            assert_eq!(account.clone().path_to_pk.is_dir(), false, "path is not a file");
        } else {
            println!("must create a new account");
        }
    }
}
