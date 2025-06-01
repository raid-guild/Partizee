use serde_json::Value;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::fs;
use serde_json::Result;
use serde::{Serialize, Deserialize};
use std::process::{Command, Output};
use std::path::PathBuf;
use crate::utils::utils::{print_output, print_error};
use crate::utils::menus::new_account_menu;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub public_key: String,
    pub address: String,
    pub network: String,
    pub path: PathBuf,
    pub account_index: u8,
    pub account_path: PathBuf,
}

impl Account {
    pub fn new( name: Option<&str>, network: Option<&str>, path_to_account: Option<&str>, public_key: Option<&str>, address: Option<&str>, account_index: Option<u8>) -> Self {
        let mut new_account: Self;
        if path_to_account.is_some() {
            new_account = load_from_file(&path_to_account.unwrap().into()).unwrap();
        } else {
            let network: String = network.unwrap_or("testnet").to_string();
            let path: PathBuf = if path_to_account.is_some() { PathBuf::from(path_to_account.unwrap()) } else { id_pbc_path().unwrap() };
            let public_key: String = public_key.unwrap_or("").to_string();
            let address: String = address.unwrap_or("").to_string(); //get_account_address(Some(&network)).unwrap();
            let account_index: u8 = account_index.unwrap_or(0);
            let name: String = name.unwrap_or(&format!("account_{}", account_index)).to_string();
            let account_path: PathBuf = path.join(format!("accounts/{}.json", name));    

        new_account = Self {
            name: name,
            network: network.clone(),
            public_key: public_key,
            address: address,
            path: path.clone(),
            account_index: account_index,
            account_path: account_path,
        };
    }
        new_account.update_account(Some(&network.unwrap()));
        new_account.save_to_file().expect("Failed to save account to file");
        new_account

    }

    pub fn load_from_file(&mut self) -> Option<&Self> {
       let loaded_account: Account = load_from_file(&self.account_path).unwrap();
       self.name = loaded_account.name;
       self.network = loaded_account.network;
       self.public_key = loaded_account.public_key;
       self.address = loaded_account.address;
       self.path = loaded_account.path;
       self.account_index = loaded_account.account_index;
       self.account_path = loaded_account.account_path;
       Some(self)
    }

    pub fn save_to_file(&self) -> std::io::Result<()> {
        let json: String = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(&self.account_path, json)
    }

    pub fn update_network(&mut self, network: &str) {
        self.network = network.to_string();
    }

    pub fn update_account(&mut self, network: Option<&str>) {
        self.public_key = self.get_public_key(network).unwrap_or(String::from(""));
        self.address = self.get_account_address(network).unwrap_or(String::from(""));
    }

    pub fn show_account(&self, network: Option<&str>) -> Option<Value> {
        let network_command: String = format!("--net={}", network.unwrap_or(&self.network));
        let account: Output;
        if let Some(id_pbc_path) = id_pbc_path() {
            account = Command::new("cargo")
                .arg("pbc")
                .arg("account")
                .arg("show")
                .arg(network_command)
                .arg(&self.address)
                .output()
                .expect("Failed to show account");
    
            if account.status.success() {
                let line = String::from_utf8_lossy(&account.stdout).to_string();
                let json_output: Value = serde_json::from_str(&line).unwrap();
                println!("Account: {:?}", json_output);
                   Some(json_output)
            } else {
                println!("Failed to show account");
                print_error(&account);
                return None;
            }
        } else {
            return None;
        }
    
    }
    pub fn mint_gas(&self) {
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
            println!("Gas minted successfully to account: {}", self.address);
        } else {
            print_error(&output);
        }
    }

    pub fn get_account_address(&self, network: Option<&str>) -> Option<String> {
        let public_key: Option<String> = self.get_public_key(network);
    
        if public_key.is_some() { 
            let mut decoded_str: String = public_key.unwrap();
            decoded_str = format!("00{}", decoded_str); // add Account prefix 00
            decoded_str.truncate(42); // trim to 21 bytes

            Some(decoded_str)
        } else {
            println!("Failed to get account address");
            return None;
        }
    }

    pub fn get_public_key(&self, network: Option<&str>) -> Option<String> {
        let account: Output;
        if let Some(id_pbc_path) = id_pbc_path() {
            account = Command::new("cargo")
                .arg("pbc")
                .arg("wallet")
                .arg("publickey")
                .arg(format!("--net={}", network.unwrap_or(&self.network)))
                .arg(format!("--path={}", id_pbc_path.display()))
                .arg("-v")
                .output()
                .expect("Failed to get public key");
    
            if account.status.success() {
                let line = String::from_utf8_lossy(&account.stdout).to_string();
                // decode base64 public key
                let decoded: Vec<u8> = STANDARD.decode(line.split(':').nth(1).unwrap().trim().to_string()).unwrap();
                let public_key: String = decoded.iter().map(|b| format!("{:02x}", b)).collect::<String>();
                return Some(public_key);
            } else {
                print_error(&account);
                return None;
            }
        } else {
            return None;
        }
    }

    pub fn create_account(&self, network: Option<&str>) -> Option<String>{
        let network_command: String = format!("--net={}", network.unwrap_or(&self.network));
        // look for existing account in .pbc folder
        if self.get_account_address(network).is_none() {
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
            } else {
                // open menu to ask if user wants to create a new account
                let force_create: bool = new_account_menu().unwrap();
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
            }
        } 

        None
    }
}

pub fn load_from_file(path: &PathBuf) -> Option<Account> {
    let data = std::fs::read_to_string(path).ok()?;
    if !data.is_empty() {
        let self_struct: Account = serde_json::from_str(&data).ok()?;
            Some(Account::new(
                Some(self_struct.name.as_ref()),
                Some(self_struct.network.as_ref()),
                self_struct.path.to_str(),
                Some(self_struct.public_key.as_ref()),
                Some(self_struct.address.as_ref()),
                Some(self_struct.account_index)))
    } else {
        println!("Failed to load account from file");
        None
    }
 
    
}

pub fn id_pbc_path() -> Option<PathBuf> {
    // Get the user's home directory
    let mut pbc_dir: PathBuf = dirs::home_dir()?;
    pbc_dir.push(".pbc");

    if !pbc_dir.is_dir() {
        return None;
    }

    pbc_dir.push("id_pbc");
    
    if pbc_dir.is_file() {
        Some(pbc_dir)
    } else {
        None
    }
}


pub fn trim_public_key(std_output: &Output) -> String {
    let line = String::from_utf8_lossy(&std_output.stdout).to_string();
    let public_key: String = line.split(':').nth(1).unwrap().trim().to_string();
    public_key
}