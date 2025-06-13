use crate::commands::user_profile::Profile;
use crate::utils::fs_nav::find_workspace_root;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::os::unix::fs::PermissionsExt;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
};
use tempfile::Builder;

/// Prints command output to console and attempts to parse as JSON
/// 
/// # Arguments
/// * `function_name` - Name of function for logging
/// * `output` - Command output to process
/// 
/// # Returns
/// * `Result<T>` - Parsed JSON output if successful
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

/// Verifies that current directory is a Partizee project
/// Checks for workspace root with required structure
/// 
/// # Returns
/// * `Result<()>` - Ok if valid project, Error otherwise
pub fn assert_partizee_project() -> Result<(), Box<dyn std::error::Error>> {
    let partizee_project: bool = find_workspace_root().is_some();
    if !partizee_project {
        return Err("Current directory is not a partizee project".into());
    }
    Ok(())
}

/// Prints error output to console and returns error
/// 
/// # Arguments
/// * `output` - Command output containing error
/// 
/// # Returns
/// * `Result<T>` - Error with stderr message
pub fn print_error<T: DeserializeOwned>(output: &Output) -> Result<T, Box<dyn std::error::Error>> {
    let line = String::from_utf8_lossy(&output.stderr).to_string();
    eprintln!("STDERR:\n{}", line);
    return Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        line,
    )));
}

/// Loads account details from a private key file
/// Validates private key and address
/// 
/// # Arguments
/// * `path` - Path to private key file
/// * `network` - Network to use
/// 
/// # Returns
/// * `Result<Profile>` - Account profile if successful
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

/// Validates blockchain address length
/// 
/// # Arguments
/// * `address` - Address to validate
/// 
/// # Returns
/// * `Result<()>` - Ok if valid length, Error otherwise
pub fn assert_address_length(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    if address.len() != 42 {
        return Err("assert_address_length: Invalid address".into());
    }
    Ok(())
}

/// Validates private key length
/// 
/// # Arguments
/// * `private_key` - Private key to validate
/// 
/// # Returns
/// * `Result<()>` - Ok if valid length, Error otherwise
pub fn assert_private_key_length(private_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    if private_key.len() != 64 {
        return Err("assert_private_key_length: Invalid private key".into());
    }
    Ok(())
}

/// Gets account address from private key file path
/// 
/// # Arguments
/// * `path` - Path to private key file
/// 
/// # Returns
/// * `Result<String>` - Account address if successful
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

/// Derives blockchain address from private key
/// Uses temporary file to call pbc command
/// 
/// # Arguments
/// * `private_key` - Private key to derive address from
/// 
/// # Returns
/// * `Result<String>` - Derived address if successful
pub fn get_address_from_pk(private_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    // validate pk length
    assert_private_key_length(private_key)?;
    // write temp file with private key
    let all_read_write = std::fs::Permissions::from_mode(0o666);
    let temp_pk = Builder::new()
        .permissions(all_read_write)
        .tempfile()
        .unwrap();
    std::fs::write(&temp_pk.path(), private_key)?;

    let mut command = Command::new("cargo");
    command.arg("pbc").arg("account").arg("address").arg(
        temp_pk
            .path()
            .canonicalize()
            .unwrap()
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
        assert_address_length(&address)?;
        Ok(address)
    } else {
        return print_error(&output.unwrap());
    }
}

/// Validates that address matches private key
/// 
/// # Arguments
/// * `address` - Address to validate
/// * `private_key` - Private key to check against
/// 
/// # Returns
/// * `Result<bool>` - True if valid, Error if validation fails
pub fn address_is_valid(
    address: &str,
    private_key: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    // validate address length
    assert_address_length(&address)?;

    // validate private key length
    assert_private_key_length(&private_key)?;

    let derived_address = get_address_from_pk(&private_key).unwrap_or("".to_string());

    // validate address length
    assert_address_length(&derived_address)?;

    if derived_address == address {
        return Ok(true);
    } else {
        return Ok(false);
    }
}

/// Creates private key file in workspace root
/// 
/// # Arguments
/// * `private_key` - Private key to save
/// 
/// # Returns
/// * `Result<PathBuf>` - Path to created file if successful
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

/// Extracts public key from command output
/// 
/// # Arguments
/// * `std_output` - Command output to parse
/// 
/// # Returns
/// * `String` - Extracted public key
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

/// Parses deployment arguments into contract-specific arguments
/// 
/// # Arguments
/// * `deploy_args` - Optional vector of deployment arguments
/// * `contracts_to_deploy` - List of contracts being deployed
/// 
/// # Returns
/// * `Option<HashMap<String, Vec<String>>>` - Map of contract names to their arguments
pub fn parse_deploy_args(
    deploy_args: Option<Vec<String>>,
    contracts_to_deploy: Vec<String>,
) -> Option<HashMap<String, Vec<String>>> {
    if deploy_args.is_some() && contracts_to_deploy.len() > 0 {
        let mut contract_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut arg_names: Vec<String> = Vec::new();
        let mut current_args: Vec<Vec<String>> = Vec::new();
        let mut sub_vector: Vec<String> = Vec::new();
        let mut current_args_index: usize = 0;
        for entry in deploy_args.unwrap().iter() {
            // iterate through args and if an arg is a contract name, split there and take the next set of args to the next contract name
            if contracts_to_deploy.contains(entry) {
                arg_names.push(entry.clone().to_lowercase());
                current_args_index += 1;
                if sub_vector.len() > 0 {
                    current_args.push(sub_vector.clone());
                }
                sub_vector.clear();
                continue;
            } else if current_args_index > 0 {
                sub_vector.push(entry.clone());
                continue;
            } else {
                return None;
            }
        }

        if sub_vector.len() > 0 {
            current_args.push(sub_vector.clone());
        }
        for (index, arg_name) in arg_names.iter().enumerate() {
            if current_args[index].len() > 0 {
                contract_map.insert(arg_name.clone(), current_args[index].clone());
            }
        }
        return Some(contract_map);
    } else {
        return None;
    }
}

/// Sets up test environment with mock files and directories
/// Creates temporary directory with Partizee project structure
/// 
/// # Returns
/// * `(TempDir, PathBuf, PathBuf)` - Temp directory, path, and original directory
#[cfg(test)]
pub fn setup_test_environment() -> (tempfile::TempDir, PathBuf, PathBuf) {
    let temp_dir = tempfile::tempdir().unwrap();
    // create a mock pk file
    let pk_file = temp_dir
        .path()
        .join("00d277aa1bf5702ab9fc690b04bd68b5a981095530.pk");
    fs::write(
        pk_file,
        "9c1a15a50a4f978f0085bd747b9da360cc0fbf5f1d0744e040873aeba46b37b0",
    )
    .expect("Failed to write mock private key file");
    // create a partizee project in the temp directory
    let partizee_project = temp_dir.path().join("rust/contracts");
    let frontend_project = temp_dir.path().join("frontend");
    let target_dir = temp_dir
        .path()
        .join("target/wasm32-unknown-unknown/release");
    fs::create_dir_all(&partizee_project).unwrap();
    fs::create_dir_all(&frontend_project).unwrap();
    fs::create_dir_all(&target_dir).unwrap();
    let cargo_toml = temp_dir.path().join("Cargo.toml");
    fs::write(cargo_toml, "[workspace]\n[package]").unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    let original_dir = std::env::current_dir().unwrap();
    (temp_dir, temp_path, original_dir) // Return temp_dir so it stays alive
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
