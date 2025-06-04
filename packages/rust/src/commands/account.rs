use serde_json::Value;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use k256::{EncodedPoint, PublicKey};
use k256::elliptic_curve::sec1::{ToEncodedPoint, FromEncodedPoint};
use k256::elliptic_curve::SecretKey;
use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};
use std::process::{Command, Output};
use std::path::PathBuf;
use crate::utils::utils::{print_output, print_error, find_workspace_root};
use crate::utils::menus::new_account_menu;
use crate::utils::constants::DEFAULT_ACCOUNT_NAME;
use crate::utils::cryptography::bip32::Bip32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub public_key: String,
    pub address: String,
    pub network: String,
    pub path_to_id_pbc: PathBuf,
    pub account_index: u8,
}

impl Default for Account {
    fn default() -> Self {
        Self {
            
            public_key: String::from(""),
            address: String::from(""),
            network: String::from("testnet"),
            path_to_id_pbc: id_pbc_path().unwrap(),
            account_index: 0,
            name: String::from(&format!("account_{}", 0)),  
        }
    }
}

impl Account {
    pub fn new( name: Option<&str>, network: Option<&str>, path_to_id_pbc: Option<&str>, public_key: Option<&str>, address: Option<&str>, account_index: Option<u8>) -> Self {
        let mut new_account: Self;
        let default_path: PathBuf = default_save_path(name.unwrap_or(&format!("{}", account_index.unwrap_or(0))));

            let network: String = network.unwrap_or("testnet").to_string();
            let path: PathBuf = if path_to_id_pbc.is_some() { PathBuf::from(path_to_id_pbc.unwrap()) } else { id_pbc_path().unwrap() };
            let public_key: String = public_key.unwrap_or("").to_string();
            let address: String = address.unwrap_or("").to_string(); //get_account_address(Some(&network)).unwrap();
            let account_index: u8 = account_index.unwrap_or(0);
            let name: String = name.unwrap_or(DEFAULT_ACCOUNT_NAME).to_string();
            let account_path: PathBuf = default_path;    

        new_account = Self {
            name: name,
            network: network.clone(),
            public_key: public_key,
            address: address,
            path_to_id_pbc: path.clone(),
            account_index: account_index,
            };
        // update public key and address
        new_account.update_account(None);
        new_account.save_to_file().expect("Failed to save account to file");
        new_account

    }

    pub fn load_from_file(&mut self) -> Option<&Self> {
        let account_path: PathBuf = default_save_path(&self.name);
       let loaded_account: Option<Account> = load_from_file(Some(account_path));
       if loaded_account.is_some() {
       *self = loaded_account.unwrap();
       Some(self)
       } else {
        println!("Failed to load account from file");
        None
       }
    }

    pub fn save_to_file(&self) -> std::io::Result<()> {
        let json: String = serde_json::to_string_pretty(self).unwrap();
        let account_path: PathBuf = default_save_path(&self.name);
        if account_path.is_file() {
            std::fs::write(&account_path, json).expect("Failed to write to file");
        } else {
            println!("Saving account to file: {}", account_path.display());
            std::fs::create_dir_all(account_path.parent().unwrap()).expect("Failed to create directory");
            std::fs::write(account_path, json).expect("Failed to write to file");
        }
        Ok(())
    }

    pub fn update_network(&mut self, network: &str) {
        self.network = network.to_string();
    }

    pub fn update_account(&mut self, network: Option<&str>) {
        self.public_key = self.get_b64_public_key(None).expect("Failed to get b64 public key");
        self.address = self.get_account_address(network).expect("Failed to get account address");
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

    pub fn get_account_address(&mut self, b64_public_key: Option<&str>) -> Option<String> {
        // check for stored key in account file
        if !self.address.is_empty() && b64_public_key.is_none() {
            // verify stored address is 21 bytes long
            if self.address.len() == 42 {
                // verify address is valid
                if self.address.starts_with("00") {
                    return Some(self.address.clone());
                }
            }
        }

        let public_key: String;

        if b64_public_key.is_some() {
            public_key = b64_public_key.unwrap().to_string();
        } else {
            // if no stored address check for stored public key
        if !self.public_key.is_empty() {
                    public_key = self.public_key.clone();
        
            } else {
                // get b64 public key from pbc
            public_key = self.get_b64_public_key(None).unwrap();
            } 
        }

        assert!(public_key.len() == 44);
        let public_key: Vec<u8> = self.get_compressed_public_key(Some(&public_key)).unwrap();

        // verify public key vec has a length  of 33
        if public_key.len() != 33 {
            println!("Invalid public key length");
            return None;
        }
        
        // Parse compressed public key
        let encoded = EncodedPoint::from_bytes(&public_key).ok()?;
        let pubkey = PublicKey::from_encoded_point(&encoded);
        // Serialize as uncompressed
        let uncompressed = pubkey.unwrap().to_encoded_point(false);
        let uncompressed_bytes = uncompressed.as_bytes();

        // Hash the uncompressed key (skip 0x04 prefix)
        let hash = Sha256::digest(&uncompressed_bytes[1..]);

        // create address add  00 to beggning and truncate to the last 20 bytes
        let mut address: String = format!("00{}", hex::encode(hash));
        // truncate to 21 bytes
        address.truncate(42);
        self.address = address.clone();
        self.save_to_file().expect("Failed to save account to file");
        Some(address)
    }

    pub fn derive_private_key(&mut self) -> Option<String> {
        let seed = self.get_compressed_public_key(None).unwrap();
        let master = Bip32::generate_master_key(&seed).unwrap();
        let child = Bip32::derive_child_key(&master, 0).unwrap();
        let private_key: SecretKey<k256::Secp256k1> = child.private_key().unwrap().clone();
        let private_key_str: String = hex::encode(&private_key.to_bytes());
        // create .pk file in project root
        self.save_pk_to_file(&private_key_str);
        Some(private_key_str)
    }

    pub fn save_pk_to_file(&mut self, pk: &str) -> Option<()> {
        let pk_path: PathBuf = self.pk_file_path();
        std::fs::write(pk_path, pk).expect("Failed to write private key to file");
        Some(())
    }

    pub fn pk_file_path(&mut self) -> PathBuf {
        let project_root: PathBuf = find_workspace_root().unwrap();
        let pk_name: &str = &self.public_key[..12];
        let pk_path: PathBuf = project_root.join(format!("{}.pk", pk_name));
        pk_path
    }

 

    // gets the b64 public key from pbc file mnemonic
    pub fn get_b64_public_key(&mut self, id_pbc_path_input: Option<PathBuf>) -> Option<String> {
        let account: Output;
        let id_pbc_path = id_pbc_path_input.unwrap_or(id_pbc_path().unwrap());
        if id_pbc_path.is_file() {
            account = Command::new("cargo")
                .arg("pbc")
                .arg("wallet")
                .arg("publickey")
                .arg(format!("--net={}", &self.network))
                .arg(format!("--path={}", id_pbc_path.display()))
                .arg("-v")
                .output()
                .expect("Failed to get public key");
                
            if account.status.success() {
                let line: String = String::from_utf8_lossy(&account.stdout).to_string();
                let b64_key: String = line.trim().split_whitespace().last().unwrap().trim().to_string(); // get the last word

                self.public_key = b64_key.clone();
                self.save_to_file().expect("Failed to save account to file");
                
                return Some(b64_key);
            } else {
                print_error(&account);
                return None;
            }

        } else {
            println!("Failed to get public key from pbc");
            return None;
        }
    }

    // should be called after get_b64_public_key or pass in b64_public_key
    pub fn get_compressed_public_key(& mut self, b64_public_key: Option<&str>) -> Option<Vec<u8>> {
        let b64_key = b64_public_key.unwrap_or(&self.public_key);
        // if no stored public key, get it from pbc
        if b64_key.is_empty() {
            let b64_public_key: String = self.get_b64_public_key(None).unwrap();
            let compressed_public_key: Vec<u8> = STANDARD.decode(b64_public_key).expect("Failed to decode public key");
            
            return Some(compressed_public_key);
        } else {
            let compressed_public_key: Vec<u8> = STANDARD.decode(b64_key).expect("Failed to decode public key");
            return Some(compressed_public_key);
        }
    }
             
    pub fn create_account(&mut self, network: Option<&str>) -> Option<String>{
        let network_command: String = format!("--net={}", network.unwrap_or(&self.network));
        // check if .pbc folder exists
        if id_pbc_path().is_some() {
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

pub fn default_save_path(name: &str) -> PathBuf {
    let mut pbc_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    pbc_dir.push(".accounts/");
    pbc_dir.push(format!("{}.json", name));
    pbc_dir
}

pub fn load_from_file(path: Option<PathBuf>) -> Option<Account> {
    let data: String;
    if path.is_some() { 
        data = std::fs::read_to_string(path.unwrap()).ok()?;
    } else {
        return None;
    }
    if !data.is_empty() {
        let self_struct: Account = serde_json::from_str(&data).ok()?;
            Some(Account::new(
                Some(self_struct.name.as_ref()),
                Some(self_struct.network.as_ref()),
                Some(self_struct.path_to_id_pbc.to_str().unwrap()),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_account() {
        let mut account: Account = Account::new(Some("test"), Some("testnet"), None, None, None, None);
        assert!(default_save_path("test").is_file());
    }

    #[test]
    fn test_load_from_file() {
        let mut account: Account = Account::new(Some("test"), Some("testnet"), None, None, None, None);
        let loaded_account: Option<Account> = load_from_file(Some(default_save_path("test")));
        assert!(loaded_account.is_some());
    }

    #[test]
    fn test_get_b64_public_key() {
        let mut account: Account = Account::new(Some("test"), Some("testnet"), None, None, None, None);
        let b64_public_key: String = account.get_b64_public_key(None).unwrap();
        assert!(b64_public_key.len() == 44);
    }

    #[test]
    fn test_get_compressed_public_key() {
        let mut account: Account = Account::new(Some("test"), Some("testnet"), None, None, None, None);
        let compressed_public_key: Vec<u8> = account.get_compressed_public_key(None).unwrap();
        assert!(compressed_public_key.len() == 33);
    }

    #[test]
    fn test_get_account_address() {
        let mut account: Account = Account::new(Some("test"), Some("testnet"), None, None, None, None);
        let address: String = account.get_account_address(None).unwrap();
        assert!(address.len() == 42);
    }

    #[test]
    fn test_derive_private_key() {
        let mut account: Account = Account::new(Some("test"), Some("testnet"), None, None, None, None);
        let private_key: String = account.derive_private_key().unwrap();
        println!("PRIVATE KEY: {}", private_key);
        assert!(private_key.len() == 64);
    }

}
