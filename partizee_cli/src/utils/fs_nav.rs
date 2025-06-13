use std::collections::HashSet;
use std::env;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use walkdir::WalkDir;

/// Finds the workspace root directory by searching for a Cargo.toml with [workspace] section
/// and required project structure (rust/contracts and frontend directories)
/// 
/// Uses multithreading to speed up the process of searching nearby directories
/// Searches up to 3 parent directories deep
/// 
/// # Returns
/// * `Option<PathBuf>` - Path to workspace root if found, None otherwise
pub fn find_workspace_root() -> Option<PathBuf> {
    let mut current_folder: PathBuf = env::current_dir().ok()?;
    for _ in 0..3 {
        let entries: Vec<_> = WalkDir::new(&current_folder)
            .max_depth(5)
            .into_iter()
            .filter_map(Result::ok)
            .collect();

        let (tx, rx) = mpsc::channel();
        let chunk_size = (entries.len() / 4).max(1);
        let found = Arc::new(AtomicBool::new(false));

        entries
            .chunks(chunk_size)
            .map(|chunk| {
                let tx = tx.clone();
                let chunk = chunk.to_vec();
                let found = Arc::clone(&found);
                thread::spawn(move || {
                    for entry in chunk {
                        if found.load(Ordering::Relaxed) {
                            return;
                        }

                        let path = entry.path();
                        if !path.is_dir() {
                            continue;
                        }

                        if let Some(files) = path.read_dir().ok() {
                            for file in files.flatten() {
                                if let Some(fname) = file.path().file_name() {
                                    if fname.to_str() == Some("Cargo.toml") {
                                        if let Ok(contents) = std::fs::read_to_string(file.path()) {
                                            if contents.contains("[workspace]")
                                                && path.join("rust/contracts").is_dir()
                                                && path.join("frontend/").is_dir()
                                            {
                                                found.store(true, Ordering::Relaxed);
                                                tx.send(Some(path.to_path_buf())).ok();
                                                return;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    tx.send(None).ok();
                })
            })
            .for_each(|handle| {
                handle.join().ok();
            });

        drop(tx);
        if let Ok(result) = rx.recv() {
            if let Some(path) = result {
                return Some(path);
            }
        }
        for result in rx {
            if let Some(path) = result {
                return Some(path);
            }
        }
        if let Some(parent) = current_folder.parent() {
            current_folder = parent.to_path_buf();
        } else {
            break;
        }
    }
    None
}

/// Finds a specific directory by name within the current directory or its parents
/// 
/// # Arguments
/// * `current_folder` - Starting directory to search from
/// * `target_folder` - Name of directory to find
/// 
/// # Returns
/// * `Option<PathBuf>` - Path to target directory if found, None otherwise
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

/// Finds all files with a specific extension in the current directory or nearby directories
/// 
/// # Arguments
/// * `starting_path` - Directory to start searching from
/// * `extension` - File extension to search for (without the dot)
/// 
/// # Returns
/// * `Vec<PathBuf>` - Vector of paths to matching files
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

/// Finds all files and directories containing a specific name
/// 
/// # Arguments
/// * `starting_path` - Directory to start searching from
/// * `name` - Name to search for (case insensitive)
/// 
/// # Returns
/// * `Vec<PathBuf>` - Vector of paths to matching files/directories
pub fn find_paths_with_name(starting_path: &PathBuf, name: &str) -> Vec<PathBuf> {
    let mut matches = Vec::new();
    let mut current_path: PathBuf = PathBuf::from(starting_path);
    let name_lowercase = name.to_lowercase();
    for _ in 0..3 {
        for entry in WalkDir::new(&current_path).max_depth(5) {
            if let Ok(entry) = entry {
                if entry.path().is_dir() {
                    let files = entry.path().read_dir().unwrap().flatten();
                    for file in files {
                        let path = file.path();
                        if path.file_name().unwrap().to_str().unwrap().to_lowercase().contains(&name_lowercase) {
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

/// Gets all contract names from compiled contract files
/// Searches for .pbc, .zkwa, and .wasm files in the release directory
/// 
/// # Returns
/// * `Option<Vec<String>>` - Vector of unique contract names if found, None if no contracts exist
pub fn get_all_contract_names() -> Option<Vec<String>> {
    let root_path: PathBuf =
        find_workspace_root().unwrap_or(PathBuf::from(env::current_dir().unwrap()));
    let path: Option<PathBuf> = find_dir(&root_path, "wasm32-unknown-unknown/release");
    if path.is_none() {
        return None;
    } else {
        let path: PathBuf = path.unwrap();
        if !path.is_dir() {
            return None;
        }
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
        Some(unique_contract_names.into_iter().collect())
    }
}

/// Finds all .pk (private key) files in the workspace and parent directories
/// Searches up to 5 parent directories deep if no files found in workspace
/// 
/// # Returns
/// * `Vec<PathBuf>` - Vector of paths to .pk files, with duplicates removed
pub fn get_pk_files() -> Vec<PathBuf> {
    let root_path: PathBuf =
        find_workspace_root().unwrap_or(PathBuf::from(env::current_dir().unwrap()));
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

/// Gets the path to the PBC identity file
/// Looks for id_pbc file in the user's home directory under .pbc folder
/// 
/// # Returns
/// * `Option<PathBuf>` - Path to id_pbc file if it exists, None otherwise
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
    use crate::utils::utils::setup_test_environment;
    use std::time::Instant;

    fn cleanup(original_dir: PathBuf) {
        std::env::set_current_dir(original_dir).unwrap();
    }
    #[test]
    fn test_find_workspace_root() {
        let (_, temp_path, original_dir) = setup_test_environment();
        // set the current directory to mock project
        std::env::set_current_dir(temp_path.join("rust/contracts")).unwrap();

        let iterations = 1000;
        let start = Instant::now();

        for _ in 0..iterations {
            let _ = find_workspace_root();
        }

        let duration = start.elapsed();
        let avg_duration = duration.as_micros() as f64 / iterations as f64;

        println!(
            "Average time for find_workspace_root: {:.2} microseconds",
            avg_duration
        );
        println!("Total time for {} iterations: {:.2?}", iterations, duration);
        assert_eq!(
            find_workspace_root().unwrap().exists(),
            true,
            "root path does not exist"
        );
        cleanup(original_dir);
    }

    #[test]
    fn test_get_all_contract_names() {
        let (_, temp_path, original_dir) = setup_test_environment();
        let _ = std::env::set_current_dir(&temp_path);
        // create a mock pbc file
        let pbc_file = temp_path.join("target/wasm32-unknown-unknown/release/counter.pbc");
        let _ = std::fs::write(&pbc_file, "");
        // create a mock zkwa file
        let zkwa_file = temp_path.join("target/wasm32-unknown-unknown/release/counter.zkwa");
        let _ = std::fs::write(&zkwa_file, "");
        // create a mock wasm file
        let contract_names = get_all_contract_names();

        cleanup(original_dir);
        assert_eq!(
            contract_names.as_ref().unwrap().len(),
            1,
            "contract_names should be 1"
        );
        assert_eq!(
            contract_names.as_ref().unwrap()[0],
            "counter",
            "contract_names should be counter"
        );
    }
}
