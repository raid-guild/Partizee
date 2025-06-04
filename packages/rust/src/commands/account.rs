use crate::utils::constants::{
    DEFAULT_ACCOUNT_INDEX, DEFAULT_ACCOUNT_NAME, DEFAULT_NETWORK,
};
use crate::utils::menus::{new_wallet_menu, select_account_menu};
use crate::utils::utils::{default_save_path, get_pk_files, load_from_file, print_error, print_output, id_pbc_path, trim_public_key, new_account_menu};
use pbc_contract_common::address::Address;
use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub private_key: Option<String>,
    pub address: Option<String>,
    pub network: String,
}

impl Default for Account {
    fn default() -> Self {
        Self {
            name: DEFAULT_ACCOUNT_NAME
                .to_string()
                .replace("{index}", &format!("{}", DEFAULT_ACCOUNT_INDEX)),
            address: None,
            private_key: None,
            network: DEFAULT_NETWORK.to_string(),
        }
    }
}

impl Account {
    pub fn new(
        name: Option<&str>,
        network: Option<&str>,
        address: Option<&str>,
        private_key: Option<&str>,
    ) -> Self {
        let new_account: Self;

        let network: String = network.unwrap_or(DEFAULT_NETWORK).to_string();
        // number of accounts in .accounts folder
        let account_count: usize = fs::read_dir(default_save_path("")).unwrap().count();
        let name: String = name
            .unwrap_or(DEFAULT_ACCOUNT_NAME)
            .to_string()
            .replace("{index}", &format!("{}", account_count));

        new_account = Self {
            name: name,
            network: network,
            private_key: private_key.map(|s| s.to_string()),
            address: address.map(|s| s.to_string()),
        };
        // verify private key is valid

        new_account
    }

    pub fn load_account(&self) -> Option<Account> {
        let pk_files: Vec<PathBuf> = get_pk_files();
        if pk_files.is_empty() {
            // interactive menu to create a new account
            let new_account: bool = new_account_menu().unwrap();
            if new_account {
                let account: Account = Account::new(None, None, None, None);
                account
                    .save_to_file()
                    .expect("Failed to save account to file");
                return Some(account);
            } else {
                return None;
            }
        } else {
            // address is file name - ext
            let file_name: String = pk_files[0]
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let address: String = file_name.split('.').nth(0).unwrap().to_string();
            // private key is file content
            let private_key: String = fs::read_to_string(pk_files[0].clone()).unwrap();
            let account: Account = Account::new(None, None, Some(&address), Some(&private_key));
            account
                .save_to_file()
                .expect("Failed to save account to file");
            return Some(account);
        }
    }

    pub fn load_from_file(&mut self, path_to_account: Option<PathBuf>) -> Result<&Self, String> {
        let account_path: PathBuf = path_to_account.unwrap_or(default_save_path(&self.name));
        let loaded_account: Option<Account> = load_from_file(Some(&account_path));
        if let Some(account) = loaded_account {
            *self = account;
            Ok(self)
        } else {
            Err(format!(
                "Failed to load account from file: {:?}",
                account_path.display()
            ))
        }
    }

    pub fn save_to_file(&self) -> std::io::Result<()> {
        let json: String = serde_json::to_string_pretty(self).unwrap();
        let account_path: PathBuf = default_save_path(&self.name);
        if account_path.is_file() {
            std::fs::write(&account_path, json).expect("Failed to write to file");
        } else {
            println!("Saving account to file: {}", account_path.display());
            std::fs::create_dir_all(account_path.parent().unwrap())
                .expect("Failed to create directory");
            std::fs::write(account_path, json).expect("Failed to write to file");
        }
        Ok(())
    }

    pub fn update_network(&mut self, network: &str) {
        self.network = network.to_string();
    }

    pub fn show_account(&self, network: Option<&str>, address: Option<&str>) -> Result<String, Box<dyn Error + 'static>> {
        let network_command = format!("--net={}", network.unwrap_or(&self.network));
        let address: Option<String> = address.map(|s| s.to_string());
        if address.is_none() {
            let pk_files: Vec<PathBuf> = get_pk_files();
            if pk_files.is_empty() {
                return Err("No account files found".into());
            } else {
                // get address from first account file
                let address: String = self.get_account_address(None).unwrap();
                let command: Output = Command::new("cargo")
                    .arg("pbc")
                    .arg("account")
                    .arg("show")
                    .arg(network_command)
                    .arg(address)
                    .output()
                    .expect("Failed to show account");

                if command.status.success() {
                   return print_output(&command);
                } else {
                    return print_error(&command);
                }
            }
        }
        let account_json: Output;
        if self.address.is_some() {
            account_json = Command::new("cargo")
                .arg("pbc")
                .arg("account")
                .arg("show")
                .arg(network_command)
                .arg(self.address.as_ref().unwrap())
                .output()
                .expect("Failed to show account");

            if account_json.status.success() {
                return print_output(&account_json);
            } else {
                return print_error(&account_json);
            }
        } else {
            return Err("Account address is not set".into());
        }
    }

    pub fn mint_gas(&self) {
        // can only mint gas on testnet
        let network_command: String = format!("--net=testnet");
        // check if address is not None
        if self.address.is_none() {
            println!("Account address is not set");
            return;
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
            println!(
                "Gas minted successfully to account: {}",
                self.address.as_ref().unwrap()
            );
        } else {
            print_error(&output);
        }
    }

    pub fn get_account_address(& self, name: Option<&str>) -> Option<String> {
        if self.address.is_none() {
            let pk_files: Vec<PathBuf> = get_pk_files();
            if pk_files.is_empty() {
                println!("No account files found");
                return None;
            } else {
                // find account file name that contains passed in name
                let account_file: Option<PathBuf> = pk_files
                    .iter()
                    .find(|file| {
                        file.file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .contains(name.unwrap())
                    })
                    .cloned();
                if account_file.is_some() {
                    let address: String = account_file
                        .unwrap()
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();
                    return Some(address);
                } else {
                    println!("Account file not found");
                    return None;
                }
            }
        } else {
            Some(self.address.as_ref().unwrap().to_string())
        }
    }

    pub fn get_private_key(&self, name: Option<&str>) -> Option<String> {
        
        if self.private_key.is_none() {
            let pk_files: Vec<PathBuf> = get_pk_files();
            if pk_files.is_empty() {
                println!("No account files found");
                return None;
            } else {
                // if name is passed in, find account file name that contains passed in name
                if name.is_some() {
                    let account_file: Option<PathBuf> = pk_files
                        .iter()
                        .find(|file| {
                            file.file_name()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .contains(name.unwrap())
                        })
                        .cloned();
                    if account_file.is_some() {
                        let private_key: String =
                            fs::read_to_string(account_file.unwrap()).unwrap();
                        Some(private_key)
                    } else {
                        println!("Account file not found");
                        return None;
                    }
                } else {
                    if pk_files.len() == 1 {
                        let private_key: String = fs::read_to_string(pk_files[0].clone()).unwrap();
                        Some(private_key)
                    } else {
                        // open menu to select account
                        let account_file: PathBuf = select_account_menu().expect("Failed to select account");
                        if account_file.is_file() {
                            let private_key: String =
                                fs::read_to_string(account_file).unwrap();
                            Some(private_key)
                        } else {
                            println!("Account file not found");
                            return None;
                        }
                    }
                }
            }
        } else {
            Some(self.private_key.as_ref().unwrap().to_string())
        }
    }

    pub fn create_account(&mut self, network: Option<&str>) -> Option<Account> {
        let network_command: String = format!("--net={}", network.unwrap_or(&self.network));
        let output = Command::new("cargo")
            .arg("pbc")
            .arg("account")
            .arg("create")
            .arg(&network_command)
            .output()
            .expect("Failed to create account");
        if !output.status.success() {
            print_error(&output);
            return None;
        } else {
            let output_line: String = String::from_utf8_lossy(&output.stdout).to_string();
            print_output(&output);
            let account: Account = self.load_account().unwrap();
            account
                .save_to_file()
                .expect("Failed to save account to file");
            Some(account)
        }
    }

    pub fn create_wallet(&mut self, network: Option<&str>) -> Option<String> {
        let network_command: String = format!("--net={}", network.unwrap_or(&self.network));
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
                // Print both stdout and stderr for full context
                print_output(&output);
                print_error(&output);
                return None;
            } else if id_pbc_path().unwrap().is_file() {
                // open menu to ask if user wants to create a new account
                let force_create: bool = new_wallet_menu().unwrap();
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
                        print_error(&output);
                        return None;
                    } else {
                        let public_key: String = trim_public_key(&output);
                        return Some(public_key);
                    }
                } else {
                    return None;
                }
            } else {
                println!("I didn't expect to get here.  I mean, the id_pbc file exists or it doesnt.  There shouldn't be a third option.");
                return None;
            }
        }

        None
    }

    // pub fn get_balance(&mut self, network: Option<&str>, token: Option<&str>) -> Option<String> {
    //     let network_command: String = format!("--net={}", network.unwrap_or(&self.network));
    //     let token_command: String = format!("--token={}", token.unwrap_or(""));
    //     // create bet balance tx

    //     let output = Command::new("cargo")
    //         .arg("pbc")
    //         .arg("transaction")
    //         .arg("send")
    //         .arg(network_command)
    //         .arg(token_command)
    //         .output()
    //         .expect("Failed to get balance");
    // }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;
    use super::*;
    static NAME: &str = "test";
    static account: LazyLock<Account> = LazyLock::new(|| Account::new(Some(NAME), Some("testnet"), None, None));
    #[test]
    fn test_create_wallet() {
        assert!(default_save_path(NAME).is_file());
    }

    #[test]
    fn test_load_from_file() {
        let loaded_account: Option<Account> = load_from_file(Some(&default_save_path(NAME)));
        assert!(loaded_account.is_some());
    }

    #[test]
    fn test_get_account_address() {
        let address: String = account.get_account_address(None).unwrap();
        assert!(address.len() == 42);
    }

    #[test]
    fn test_get_private_key() {
        let private_key: String = account.get_private_key(Some(NAME)).unwrap();
        assert!(private_key.len() == 64);
    }
}
