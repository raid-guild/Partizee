use crate::commands::account::Account;
use crate::utils::fs_nav::find_workspace_root;
use serde::de::DeserializeOwned;
use rand::Rng;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::LazyLock,
};

pub static COPIABLE_EXTENSIONS: LazyLock<Vec<&str>> = LazyLock::new(|| {
    vec![
        ".js", ".jsx", ".ts", ".tsx", ".json", ".ico", ".png", ".svg", ".jpg", ".jpeg", ".gif",
        ".webp", ".bmp", ".tiff", ".tif", ".ico", ".cur", ".ani", ".avif", ".heic", ".heif",
        ".webp",
    ]
});

/// print output to console
/// return json output
pub fn print_output<T: DeserializeOwned>(
    function_name: &str,
    output: &Output,
) -> Result<T, Box<dyn std::error::Error>> {
    let line = String::from_utf8_lossy(&output.stdout).to_string();
    println!("function_name: {} \n STDOUT:\n{}", function_name, line);
    let json_output: Result<T, serde_json::Error> = serde_json::from_str(&line);
    match json_output {
        Ok(val) => Ok(val),
        Err(e) => Err(Box::new(e)),
    }
}

/// print error to console
/// return error
pub fn print_error<T: DeserializeOwned>(output: &Output) -> Result<T, Box<dyn std::error::Error>> {
    let line = String::from_utf8_lossy(&output.stderr).to_string();
    eprintln!("STDERR:\n{}", line);
    return Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        line,
    )));
}

pub fn load_account_from_pk_file(
    path: &PathBuf,
    network: &str,
) -> Result<Account, Box<dyn std::error::Error>> {
    if !path.is_file() {
        return Err(format!("load_account_from_pk_file: Invalid path: {}", path.display()).into());
    }
    let private_key: String = std::fs::read_to_string(path).unwrap_or_else(|e| {
        return format!("load_account_from_pk_file: Failed to read file: {}", e).into();
    });
    if private_key.is_empty() {
        return Err(format!("load_account_from_pk_file: Private key is empty").into());
    }
    assert_private_key_length(&private_key).expect("load_account_from_pk_file: Invalid private key");
    // get address from file name - remove extension

        let file_name: String = path.file_name().unwrap().to_str().unwrap().to_string();
        let mut address: String = file_name.split('.').nth(0).unwrap().to_string();


        let valid_address = address_is_valid(&address, &private_key).unwrap_or(false);

        if !valid_address {
            address = get_address_from_pk(&private_key).expect("load_account_from_pk_file: Failed to get address from private key");
            assert_address_length(&address).expect("load_account_from_pk_file: Invalid address");
        }  

        let valid_address = address_is_valid(&address, &private_key).unwrap_or(false);

        if !valid_address {
            return Err("load_account_from_pk_file: Invalid private key".into());
        }
        let account: Account = Account {
            network: network.to_string(),
            address: address,
            private_key: private_key,
            path: path.to_path_buf(),
        };
        Ok(account)
}

pub fn assert_address_length(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    if address.len() != 42 {
        return Err("assert_address_length: Invalid address".into());
    }
    Ok(())
}

pub fn assert_private_key_length(private_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    if private_key.len() != 64 {
        return Err("assert_private_key_length: Invalid private key".into());
    }
    Ok(())
}

pub fn get_account_address_from_path(path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    if path.is_file() {
        // get account address from path name account is the last word in path - remove extension path could be absolute or relative
        let account_name: String = path.file_name().unwrap().to_str().unwrap().to_string();
        let account_vec: Vec<String> = account_name.split('.').map(|s| s.to_string()).collect();
        let account_address_with_extension: String = account_vec.last().unwrap().to_string();
        let account_address: String = account_address_with_extension
            .split('.')
            .nth(0)
            .unwrap()
            .to_string();
        return Ok(account_address);
    } else {
        return Err("get_account_address_from_path: Invalid path provided".into());
    }
}

pub fn get_address_from_pk(private_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    // validate pk length
    if private_key.len() != 64 {
        return Err("get_address_from_pk: Invalid private key".into());
    }
    let root_path: PathBuf = find_workspace_root().expect("get_address_from_pk: Failed to find workspace root");
    
    // write temp file with private key 
    // create a random string for the temp file name
    let temp_file_name: String = format!("temp_{}.pk", rand::thread_rng().gen_range(1..=1000000));
    let temp_file: PathBuf = root_path.join(temp_file_name);

    let write_result = fs::write(&temp_file, private_key);

    if write_result.is_err() {
        return Err("get_address_from_pk: Failed to write private key to temp file".into());
    }

    let output: Result<Output, std::io::Error> = Command::new("cargo")
        .arg("pbc")
        .arg("account")
        .arg("address")
        .arg(&temp_file.as_path().to_str().unwrap())
        .output();

    // remove temp file
    if temp_file.exists() {
        fs::remove_file(&temp_file).unwrap();
    }
    
    if output.is_ok() {
        // get address from command output
        let mut address: String = String::from_utf8_lossy(&output.as_ref().unwrap().stdout).to_string();
        
        // trim non alphanumeric characters
        address = address.chars().filter(|c| c.is_alphanumeric()).collect();
        // validate address length
        if address.len() != 42 {
            return print_output("get_address_from_pk", &output.unwrap().clone());
        }
        Ok(address)
    } else {
        return print_error(&output.unwrap());
    }
}

pub fn address_is_valid(
    address: &str,
    private_key: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    // validate address length
    if address.len() != 42 {
        return Ok(false);
    }

    let derived_address = if let Ok(addr) = get_address_from_pk(&private_key) {
        addr
    } else {
        return Err(format!("address_is_valid: Failed to get address from private key: {}", private_key).into());
    };

    // validate address length
    if derived_address.len() != 42 {
        return Ok(false);
    }

    if derived_address == address {
        return Ok(true);
    } else {
        return Ok(false);
    }
}

pub fn create_pk_file(private_key: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let root_path: PathBuf = find_workspace_root().expect("create_pk_file: Failed to find workspace root");
        let address: String = get_address_from_pk(private_key).expect("create_pk_file: Failed to get address from private key");
        let pk_file: PathBuf = root_path.join(format!("{}.pk", address));
        fs::write(&pk_file, private_key).unwrap();
        Ok(pk_file)
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
    fn test_validate_address() {
        // validate address
        let valid_address = address_is_valid(
            "00d277aa1bf5702ab9fc690b04bd68b5a981095530",
            "9c1a15a50a4f978f0085bd747b9da360cc0fbf5f1d0744e040873aeba46b37b0",
        ).unwrap();
        assert_eq!(valid_address, true, "failed to validate address");
    }

    #[test]
    fn test_get_address_from_pk() {
        let result = get_address_from_pk(
            "9c1a15a50a4f978f0085bd747b9da360cc0fbf5f1d0744e040873aeba46b37b0",
        );
        println!("result: {:?}", result);
        assert_eq!(result.unwrap(), "00d277aa1bf5702ab9fc690b04bd68b5a981095530", "address is not correct");
    }
}
