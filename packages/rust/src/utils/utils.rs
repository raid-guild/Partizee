use std::process::{Command, Output};
use std::sync::LazyLock;
use serde::de::DeserializeOwned;
use crate::commands::account::Account;
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

pub fn find_path_with_name(folder: &Path, name: &str) -> Vec<PathBuf> {
    let mut matches = Vec::new();
    if let Ok(entries) = fs::read_dir(folder) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if file_name == name {
                    matches.push(path);
                }
            }
        }
    }
    matches
}

pub fn print_output<T: DeserializeOwned>(output: &Output) -> Result<T, Box<dyn std::error::Error>> {
    let line = String::from_utf8_lossy(&output.stdout).to_string();
    let json_output = serde_json::from_str(&line).unwrap();
    println!("STDOUT:\n{}", line);
    Ok(json_output)
}

pub fn print_error<T: DeserializeOwned>(output: &Output) -> Result<T, Box<dyn std::error::Error>> {
    let line = String::from_utf8_lossy(&output.stderr).to_string();
    eprintln!("STDERR:\n{}", line);
    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, line)));
}

pub fn default_save_path(name: &str) -> PathBuf {
    let mut pbc_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    pbc_dir.push(".accounts/");
    pbc_dir.push(format!("{}.json", name));
    pbc_dir
}

pub fn load_from_file(path: Option<&PathBuf>) -> Option<Account> {
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
            Some(self_struct.address.as_ref().unwrap()),
            Some(self_struct.private_key.as_ref().unwrap()),
        ))
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

pub fn validate_address(address: &str, private_key: &str) -> Result<bool, Box<dyn std::error::Error>> {
    // validate pk length
    if private_key.len() != 64 {
        return Err("Invalid private key".into());
    }
    let root_path: PathBuf = find_workspace_root().unwrap();
    // write temp file with private key
    let temp_file: PathBuf = root_path.join(format!("temp.pk"));
    fs::write(&temp_file, private_key).unwrap();

    let output: Result<Output, std::io::Error>   = Command::new("cargo")
        .arg("pbc")
        .arg("account")
        .arg("show")
        .arg(&temp_file.as_path().to_str().unwrap())
        .output();
    
    // remove temp file
    fs::remove_file(&temp_file).unwrap();
    // check if output is ok
    if output.is_ok() {
    // get address from command output
    let mut address_output: String = String::from_utf8_lossy(&output.unwrap().stdout).to_string().split(':').nth(1).unwrap().trim().to_string();
    // trim % off end if it exists
    address_output = address_output.trim_end_matches('%').to_string();


    // validate address length
    if address_output.len() != 42 {
        return Err("Invalid address".into());
    }
    if address_output != address {
        return Err("Address mismatch".into());
    }

    Ok(true)
    } else {
        return print_error(&output.unwrap());
    }
}

pub fn create_pk_file(private_key: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // validate pk length
    if private_key.len() != 64 {
        return Err("Invalid private key".into());
    }
    let root_path: PathBuf = find_workspace_root().unwrap();
    // write temp file with private key
    let temp_file: PathBuf = root_path.join(format!("temp.pk"));
    fs::write(&temp_file, private_key).unwrap();
    let output: Result<Output, std::io::Error>   = Command::new("cargo")
        .arg("pbc")
        .arg("account")
        .arg("show")
        .arg(&temp_file.as_path().to_str().unwrap())
        .output();
    if output.is_ok() {
    // get address from command output
    let address: String = String::from_utf8_lossy(&output.unwrap().stdout).to_string().split(':').nth(1).unwrap().trim().to_string();
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
    let root_path: PathBuf = find_workspace_root().unwrap();
    let pk_files: Vec<PathBuf> = fs::read_dir(root_path)
        .unwrap()
        .filter_map(|entry| {
            let path: PathBuf = entry.unwrap().path();
            if path.is_file() && path.extension().unwrap_or_default() == "pk" {
                Some(path)
            } else {
                None
            }
        })
        .collect();
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
}
