use crate::{commands::user_profile::Profile, utils::fs_nav::id_pbc_path};
use crate::utils::fs_nav::find_workspace_root;
use tempfile::{Builder};
use serde::de::DeserializeOwned;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
};
use std::os::unix::fs::PermissionsExt;

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

pub fn assert_partizee_project() -> Result<(), Box<dyn std::error::Error>> {
    let partizee_project: bool = find_workspace_root().is_some();
    if !partizee_project {
        return Err("Current directory is not a partizee project".into());
    }
    Ok(())
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
) -> Result<Profile, Box<dyn std::error::Error>> {
    if !path.is_file() {
        return Err(format!(
            "load_account_from_pk_file: Failed to read file: {}",
            path.display()
        )
        .into());
    }
    let private_key: String = std::fs::read_to_string(path)
        .map_err(|e| format!("load_account_from_pk_file: Failed to read file: {}", e))?;
    if private_key.is_empty() {
        return Err("load_account_from_pk_file: Private key is empty".into());
    }
    assert_private_key_length(&private_key)
        .expect("load_account_from_pk_file: Invalid private key");
    // get address from file name - remove extension

    let file_name = path
        .file_name()
        .ok_or("Invalid file path")?
        .to_str()
        .ok_or("Invalid UTF-8 in filename")?;
    let mut address = file_name
        .split('.')
        .next()
        .ok_or("Empty filename")?
        .to_string();

    let valid_address = address_is_valid(&address, &private_key).unwrap_or(false);

    if !valid_address {
        address = get_address_from_pk(&private_key)
            .expect("load_account_from_pk_file: Failed to get address from private key");
        assert_address_length(&address).expect("load_account_from_pk_file: Invalid address");
    }

    let valid_address = address_is_valid(&address, &private_key).unwrap_or(false);

    if !valid_address {
        return Err("load_account_from_pk_file: Invalid private key".into());
    }
    let account: Profile = Profile {
        network: network.to_string(),
        address: address,
        private_key: private_key,
        path_to_pk: path.to_path_buf(),
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

#[allow(dead_code)]
pub fn get_account_address_from_path(path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    if path.is_file() {
        // get account address from path name account is the last word in path - remove extension path could be absolute or relative
        let file_name = path
            .file_name()
            .ok_or("Invalid file path")?
            .to_str()
            .ok_or("Invalid UTF-8 in filename")?;

        // Remove extension to get address
        let account_address = file_name
            .split('.')
            .next()
            .ok_or("Empty filename")?
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

    // write temp file with private key
    let all_read_write = std::fs::Permissions::from_mode(0o666);
    let temp_pk = Builder::new().permissions(all_read_write).tempfile().unwrap();
    std::fs::write(&temp_pk.path(), private_key).unwrap();

    let mut command = Command::new("cargo");
    command
        .arg("pbc")
        .arg("account")
        .arg("address")
        .arg(
            temp_pk.path().canonicalize().unwrap()
                .to_str()
                .ok_or("Invalid UTF-8 in path")?,
        );
        
    let output = command.output();
    std::fs::remove_file(temp_pk.path()).unwrap();
    if !output.as_ref().unwrap().status.success() {
        let stderr = String::from_utf8_lossy(&output.as_ref().unwrap().stderr);
        return Err(format!("Command failed: {}", stderr).into());
    }
    
    if output.is_ok() {
        // get address from command output
        let mut address: String =
            String::from_utf8_lossy(&output.as_ref().unwrap().stdout).to_string();
        // trim non alphanumeric characters
        address = address.chars().filter(|c| c.is_alphanumeric()).collect();
        // validate address length
        if address.len() != 42 {
            return Err(format!("Invalid address length: {} (expected 42)", address.len()).into());
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

    // validate private key length
    if private_key.len() != 64 {
        return Ok(false);
    }

    let derived_address = get_address_from_pk(&private_key).unwrap_or("".to_string());


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
    let root_path: PathBuf =
        find_workspace_root().expect("create_pk_file: Failed to find workspace root");
    let address: String = get_address_from_pk(private_key)
        .expect("create_pk_file: Failed to get address from private key");
    let pk_file: PathBuf = root_path.join(format!("{}.pk", address));
    fs::write(&pk_file, private_key)
        .map_err(|e| format!("Failed to write private key file: {}", e))?;
    Ok(pk_file)
}
#[allow(dead_code)]
pub fn trim_public_key(std_output: &Output) -> String {
    let line = String::from_utf8_lossy(&std_output.stdout).to_string();
    let public_key = line
        .split(':')
        .nth(1)
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| {
            eprintln!("Warning: Unable to parse public key from output: {}", line);
            String::new()
        });
    public_key
}

// Add this at module level, before the tests
#[cfg(test)]
pub fn setup_test_environment() -> (tempfile::TempDir, PathBuf, PathBuf) {
    let temp_dir = tempfile::tempdir().unwrap();
    // create a mock pk file
    let pk_file = temp_dir.path().join("00d277aa1bf5702ab9fc690b04bd68b5a981095530.pk");
    fs::write(pk_file, "9c1a15a50a4f978f0085bd747b9da360cc0fbf5f1d0744e040873aeba46b37b0").expect("Failed to write mock private key file");
    // create a partizee project in the temp directory
    let partizee_project = temp_dir.path().join("rust/contracts");
    let frontend_project = temp_dir.path().join("frontend");
    fs::create_dir_all(&partizee_project).unwrap();
    fs::create_dir_all(&frontend_project).unwrap();
    let cargo_toml = temp_dir.path().join("Cargo.toml");
    fs::write(cargo_toml, "[workspace]\n[package]").unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    let original_dir = std::env::current_dir().unwrap();
    (temp_dir, temp_path, original_dir)  // Return temp_dir so it stays alive
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
        )
        .unwrap();
        assert_eq!(valid_address, true, "failed to validate address");
    }

    #[test]
    fn test_get_address_from_pk() {
        let result =
            get_address_from_pk("9c1a15a50a4f978f0085bd747b9da360cc0fbf5f1d0744e040873aeba46b37b0");
        println!("result: {:?}", result);
        assert_eq!(
            result.unwrap_or("".to_string()),
            "00d277aa1bf5702ab9fc690b04bd68b5a981095530",
            "address is not correct"
        );
    }
}
