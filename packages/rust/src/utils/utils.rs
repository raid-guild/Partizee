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
    let mut dir = Some(env::current_dir().unwrap());
    let mut depth = 0;
    // limit max depth, cause we don't want to search the whole filesystem
    let max_depth = 5;

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
        dir = Some(current.parent().unwrap().to_path_buf());
        depth += 1;
    }
    None
}

pub fn find_paths_with_extension(folder: &Path, extension: &str) -> Vec<PathBuf> {
    let mut matches = Vec::new();
    if let Ok(entries) = fs::read_dir(folder) {
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
