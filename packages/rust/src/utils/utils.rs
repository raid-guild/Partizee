use std::fs;
use std::path::{Path, PathBuf};


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
        dir = current.parent();
        depth += 1;
    }
    None
}