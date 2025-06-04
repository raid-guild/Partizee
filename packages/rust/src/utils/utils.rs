use std::sync::LazyLock;
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use std::process::Output;

pub static COPIABLE_EXTENSIONS: LazyLock<Vec<&str>> = LazyLock::new(|| {
    vec![
        ".js", ".jsx", ".ts", ".tsx", ".json", ".ico", ".png", ".svg", ".jpg", ".jpeg", ".gif", ".webp",
        ".bmp", ".tiff", ".tif", ".ico", ".cur", ".ani", ".avif", ".heic", ".heif", ".webp",
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

pub fn print_output(output: &Output) {
    eprintln!("STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
}

pub fn print_error(output: &Output) {
    eprintln!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));
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
