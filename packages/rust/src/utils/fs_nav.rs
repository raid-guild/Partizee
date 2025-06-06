use std::collections::HashSet;
use std::env;
use std::path::PathBuf;
use walkdir::WalkDir;

/// note: this function is used to find the root of the workspace, it will search up to 3 levels of directories for a Cargo.toml file with the workspace flag
pub fn find_workspace_root() -> Option<PathBuf> {
    let mut current_folder: PathBuf = env::current_dir().ok()?;
    for _ in 0..3 {
        for entry in WalkDir::new(&current_folder).max_depth(5) {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    let files = path.read_dir().ok()?.flatten();
                    for file in files {
                        if file.path().file_name()?.to_str()? == "Cargo.toml" {
                            let contents = std::fs::read_to_string(file.path()).ok()?;
                            if contents.contains("[workspace]") {
                                return Some(path.to_path_buf());
                            }
                        }
                    }
                }
            }
        }
        // Move up one directory
        if let Some(parent) = current_folder.parent() {
            current_folder = parent.to_path_buf();
        } else {
            break;
        }
    }
    None
}

pub fn find_dir(current_folder: &PathBuf, target_folder: &str) -> Option<PathBuf> {
    let mut current_folder: PathBuf = current_folder.clone();
    let target_folder: PathBuf = PathBuf::from(target_folder);
    for _ in 0..3 {
        for entry in WalkDir::new(&current_folder).max_depth(5) {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.ends_with(&target_folder) && path.is_dir() {
                    return Some(path.to_path_buf());
                }
            }
        }
        // Move up one directory
        if let Some(parent) = current_folder.parent() {
            current_folder = parent.to_path_buf();
        } else {
            break;
        }
    }
    None
}

/// find paths with extension in folder or nearby folders
/// return vector of paths with selected extension
pub fn find_files_with_extension(starting_path: &PathBuf, extension: &str) -> Vec<PathBuf> {
    let mut matches = Vec::new();
    let mut current_path: PathBuf = starting_path.clone();
    for _ in 0..3 {
        for entry in WalkDir::new(&current_path).max_depth(5) {
            if let Ok(entry) = entry {
                if entry.path().is_dir() {
                    let files = entry.path().read_dir().unwrap().flatten();
                    for file in files {
                        let path = file.path();
                        if path.extension().unwrap_or_default() == extension {
                            matches.push(path);
                        }
                    }
                }
            }
        }
        if matches.is_empty() {
            if let Some(parent) = current_path.parent() {
                current_path = parent.to_path_buf();
            } else {
                break;
            }
        }
    }
    matches
}

/// find path with name in folder
/// return vector of paths
pub fn find_paths_with_name(starting_path: &PathBuf, name: &str) -> Vec<PathBuf> {
    let mut matches = Vec::new();
    let mut current_path: PathBuf = PathBuf::from(starting_path);
    for _ in 0..3 {
        for entry in WalkDir::new(&current_path).max_depth(5) {
            if let Ok(entry) = entry {
                if entry.path().is_dir() {
                    let files = entry.path().read_dir().unwrap().flatten();
                    for file in files {
                        let path = file.path();
                        if path.file_name().unwrap().to_str().unwrap().contains(name) {
                            matches.push(path);
                        }
                    }
                }
            }
        }
        if matches.is_empty() {
            if let Some(parent) = current_path.parent() {
                current_path = parent.to_path_buf();
            } else {
                break;
            }
        }
    }
    matches
}
// to be used during deployment to get all contract names
pub fn get_all_contract_names() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let path: PathBuf = find_dir(
        &find_workspace_root().unwrap(),
        "wasm32-unknown-unknown/release",
    )
    .unwrap();
    // all
    let pbc_names: Vec<String> = find_files_with_extension(&path, "pbc")
        .into_iter()
        .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
        .collect();
    let zkwa_names: Vec<String> = find_files_with_extension(&path, "zkwa")
        .into_iter()
        .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
        .collect();

    let wasm_names: Vec<String> = find_files_with_extension(&path, "wasm")
        .into_iter()
        .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
        .collect();

    // trim extensions and remove duplicates from names and return vector of names
    let contract_names: Vec<String> = pbc_names
        .into_iter()
        .chain(zkwa_names.into_iter())
        .chain(wasm_names.into_iter())
        .map(|name| {
            std::path::Path::new(&name)
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        })
        .collect();
    let unique_contract_names: HashSet<String> = contract_names.into_iter().collect();
    Ok(unique_contract_names.into_iter().collect())
}

pub fn get_pk_files() -> Vec<PathBuf> {
    let root_path: PathBuf = find_workspace_root().unwrap();
    let mut pk_files_vec: Vec<PathBuf> = find_files_with_extension(&root_path, "pk");
    if pk_files_vec.is_empty() {
        let mut depth = 0;
        let mut outer_path: PathBuf = root_path.clone();
        loop {
            outer_path = outer_path
                .parent()
                .unwrap_or(&PathBuf::from(""))
                .to_path_buf();
            if outer_path.is_dir() {
                pk_files_vec = find_files_with_extension(&outer_path, "pk");
                depth += 1;
                if depth > 5 {
                    break;
                }
                if !pk_files_vec.is_empty() {
                    break;
                }
            } else {
                break;
            }
        }
    }
    // filter duplicates
    let pk_files_set: HashSet<PathBuf> = pk_files_vec.into_iter().collect();
    let pk_files: Vec<PathBuf> = pk_files_set.into_iter().collect();
    pk_files
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_workspace_root() {
        let workspace_root = find_workspace_root();
        assert_eq!(workspace_root.unwrap().join("Cargo.toml").exists(), true);
    }
}
