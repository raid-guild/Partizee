use crate::commands::account::Account;
use serde::de::DeserializeOwned;
use std::process::{Command, Output};
use std::sync::LazyLock;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub static COPIABLE_EXTENSIONS: LazyLock<Vec<&str>> = LazyLock::new(|| {
    vec![
        ".js", ".jsx", ".ts", ".tsx", ".json", ".ico", ".png", ".svg", ".jpg", ".jpeg", ".gif",
        ".webp", ".bmp", ".tiff", ".tif", ".ico", ".cur", ".ani", ".avif", ".heic", ".heif",
        ".webp",
    ]
});

pub fn find_workspace_root() -> Option<PathBuf> {
    let mut dir: Option<PathBuf> = Some(PathBuf::from(env!("CARGO_MANIFEST_DIR")).to_path_buf());
    let mut depth = 0;
    // limit max depth, cause we don't want to search the whole filesystem
    let max_depth = 5;
    println!("Searching for workspace root in: {:?}", dir);
    while let Some(current) = dir {
        if depth >= max_depth {
            break;
        }
        let candidate = current.join("Cargo.toml");
        if candidate.exists() {
            if let Ok(contents) = fs::read_to_string(&candidate) {
                if contents.contains("[workspace]") {
                    return Some(current.to_path_buf());
                }
            }
        }
        dir = current.parent().map(|p| p.to_path_buf());
        depth += 1;
    }
    None
}

/// find paths with extension in folder
/// return vector of paths with selected extension
pub fn find_paths_with_extension(relative_path_to_folder: &Path, extension: &str) -> Vec<PathBuf> {
    let mut matches = Vec::new();
    let folder_path = PathBuf::from(relative_path_to_folder);
    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == extension {
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        matches.push(path);
                    }
                }
            }
        }
    }
    matches
}

/// Recursively search for a `target/wasm32-unknown-unknown/release` directory from the given root.
/// Returns the first found path, or an error if not found.
pub fn find_wasm_release_folder(project_root: &PathBuf) -> Result<PathBuf, String> {
    println!("Searching for wasm release folder in: {:?}", project_root);
    for entry in walkdir::WalkDir::new(project_root)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if path.ends_with("wasm32-unknown-unknown/release")
            && path.is_dir()
            && path
                .ancestors()
                .any(|ancestor| ancestor.file_name().map_or(false, |n| n == "target"))
        {
            return std::fs::canonicalize(path)
                .map_err(|e| format!("Failed to canonicalize found path: {}", e));
        }
    }
    Err("No target/wasm32-unknown-unknown/release directory found".to_string())
}

/// find path with name in folder
/// return vector of paths
pub fn find_paths_with_name(folder: &Path, name: &str) -> Vec<PathBuf> {
    let mut matches = Vec::new();
    if let Ok(entries) = fs::read_dir(folder) {
        for entry in entries.flatten() {
            let path = entry.path();
            // if file name contains name, add to matches
            if path.file_name().unwrap().to_str().unwrap().contains(name) {
                matches.push(path);
            }
        }
    }
    matches
}

/// print output to console
/// return json output
pub fn print_output<T: DeserializeOwned>(output: &Output) -> Result<T, Box<dyn std::error::Error>> {
    let line = String::from_utf8_lossy(&output.stdout).to_string();
    println!("STDOUT:\n{}", line);
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

pub fn default_save_path(name: &str) -> PathBuf {
    let mut pbc_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    pbc_dir.push(".accounts/");
    pbc_dir.push(format!("{}.json", name));
    pbc_dir
}

pub fn load_account_from_pk_file(
    path: &PathBuf,
    network: &str,
) -> Result<Account, Box<dyn std::error::Error>> {
    let data: String;
    if path.is_file() {
        data = std::fs::read_to_string(path).expect("Failed to read file");
    } else {
        return Err("Path is not a file".into());
    }
    if !data.is_empty() {
        // get address from file name - remove extension
        let file_name: String = path.file_name().unwrap().to_str().unwrap().to_string();
        let address: String = file_name.split('.').nth(0).unwrap().to_string();
        // get private key from file content
        let private_key: String = std::fs::read_to_string(path).expect("Failed to read file");
        // validate address
        let valid_inputs: bool = validate_address(&address, &private_key).unwrap();

        if !valid_inputs {
            return Err("Invalid address or private key".into());
        }

        let account: Account = Account::new(
            Some(path),
            Some(&network),
            Some(&address),
            Some(&private_key),
        )
        .unwrap();
        Ok(account)
    } else {
        return Err("Path is not a file".into());
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
        return Err("Invalid path provided".into());
    }
}

pub fn get_address_from_pk(private_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    // validate pk length
    if private_key.len() != 64 {
        return Err("Invalid private key".into());
    }
    let root_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));;
    // write temp file with private key
    let temp_file: PathBuf = root_path.join(format!("temp.pk"));
    fs::write(&temp_file, private_key).unwrap();
    let output: Result<Output, std::io::Error> = Command::new("cargo")
        .arg("pbc")
        .arg("account")
        .arg("address")
        .arg(&temp_file.as_path().to_str().unwrap())
        .output();
    println!("output: {:?}", output);
    if output.is_ok() {
        // get address from command output
        let address: String = String::from_utf8_lossy(&output.unwrap().stdout).to_string();
        // trim non alphanumeric characters
        let address: String = address.chars().filter(|c| c.is_alphanumeric()).collect();
        // validate address length
        if address.len() != 42 {
            return Err("Invalid address".into());
        }
        Ok(address)
    } else {
        return Err("Invalid private key".into());
    }
}

pub fn validate_address(
    address: &str,
    private_key: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    // validate pk length
    if private_key.len() != 64 {
        return Err("Invalid private key".into());
    }
    // validate address length
    if address.len() != 42 {
        return Err("Invalid address".into());
    }

    let derived_address: String = get_address_from_pk(&private_key).unwrap();

    // validate address length
    if derived_address.len() != 42 {
        return Err("Invalid address".into());
    }

    if derived_address == address {
        return Ok(true);
    }

    Ok(false)
}

pub fn create_pk_file(private_key: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // validate pk length
    if private_key.len() != 64 {
        return Err("Invalid private key".into());
    }
    let root_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // write temp file with private key
    let temp_file: PathBuf = root_path.join(format!("temp.pk"));
    fs::write(&temp_file, private_key).unwrap();
    let output: Result<Output, std::io::Error> = Command::new("cargo")
        .arg("pbc")
        .arg("account")
        .arg("address")
        .arg(&temp_file.as_path().to_str().unwrap())
        .output();

    if output.is_ok() {
        // get address from command output
        let address: String = String::from_utf8_lossy(&output.unwrap().stdout)
            .to_string()
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .to_string();
        // validate address length
        if address.len() != 42 {
            return Err("Invalid address".into());
        }
        let pk_file: PathBuf = root_path.join(format!("{}.pk", address));
        fs::write(&pk_file, private_key).unwrap();
        // remove temp file
        fs::remove_file(&temp_file).unwrap();
        Ok(pk_file)
    } else {
        return print_error(&output.unwrap());
    }
}

pub fn get_pk_files() -> Vec<PathBuf> {
    let root_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let pk_files: Vec<PathBuf> = find_paths_with_extension(&root_path, "pk");
    pk_files
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
    fn test_find_workspace_root() {
        let workspace_root = find_workspace_root();
        assert_eq!(workspace_root.unwrap().join("Cargo.toml").exists(), true);
    }

    #[test]
    fn test_validate_address() {
        // validate address
        let result = validate_address(
            "004f687f1eedab29dfa2cffc51739eadcbdbed2efa",
            "b47a4f0c769da7e49508bd48577a73c1c67aefdc866b9c600ceb5c69f35ff769",
        );
        assert_eq!(result.is_ok(), true);
    }
}
