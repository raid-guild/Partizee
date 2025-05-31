use serde_json::Value;
use walkdir::WalkDir;
use pbc_contract_common::address::AddressType;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::future::poll_fn;
use std::env::current_dir;
use std::process::{Command, Output};
use std::path::PathBuf;
use crate::utils::utils::{print_output, print_error};

#[derive(Debug, Clone)]
pub struct Account {
    pub public_key: String,
    pub address: String,
    pub network: String,
    pub path: PathBuf,
}

impl Account {
    pub fn new( network: Option<&str>) -> Self {

        let network: String = network.unwrap_or("testnet").to_string();
        let path: PathBuf = id_pbc_path().unwrap();
        let public_key: String = String::from("");
        let address: String = String::from(""); //get_account_address(Some(&network)).unwrap();
        

        let mut new_account: Account = Self {
            network: network.clone(),
            public_key: public_key,
            address: address,
            path: path.clone(),
        };

        new_account.update_account(Some(&network));
        new_account
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
                .arg(&self.public_key)
                .arg("--balance")
                .output()
                .expect("Failed to show account");
    
            if account.status.success() {
                let line = String::from_utf8_lossy(&account.stdout).to_string();
                let json_output: Value = serde_json::from_str(&line).unwrap();
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

    pub fn get_account_address(&self, network: Option<&str>) -> Option<String> {
        let public_key: Option<String> = self.get_public_key(network);
    
        if public_key.is_some() { 
            // base64 decode public key
            let decoded: Vec<u8> = STANDARD.decode(public_key.unwrap()).unwrap();
            let mut decoded_str: String = decoded.iter().map(|b| format!("{:02x}", b)).collect::<String>();
            decoded_str = format!("00{}", decoded_str); // add Account prefix 00
            decoded_str.truncate(42); // trim to 21 bytes
            println!("Address (hex, 42 chars): {}", decoded_str);
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
                let public_key: String = line.split(':').nth(1).unwrap().trim().to_string();
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
        // look for existing account in .pbc folder
        if !self.get_account_address(network).is_some() {
            // create new account
            let network_command: String = format!("--net={}", network.unwrap_or(&self.network));
            let output = Command::new("cargo")
                .arg("pbc")
                .arg("wallet")
                .arg("create")
                .arg(network_command)
                .output()
                .expect("Failed to create account");
            
    
            if !output.status.success() {
            // Print both stdout and stderr for full context
            print_output(&output);
            print_error(&output);
                return None;
            } else {
                // open menu to ask if user wants to create a new account
                
                let public_key: String = trim_public_key(&output);
                return Some(public_key);
            }
        } 

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