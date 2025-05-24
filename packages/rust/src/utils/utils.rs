use std::sync::LazyLock;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub static COPIABLE_EXTENSIONS: LazyLock<Vec<&str>> = LazyLock::new(|| {
    vec![
        ".js", ".jsx", ".ts", ".tsx", ".json", "ico", "png", "svg", "jpg", "jpeg", "gif", "webp",
        "bmp", "tiff", "tif", "ico", "cur", "ani", "avif", "heic", "heif", "webp",
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_workspace_root() {
        let workspace_root = find_workspace_root();
        assert_eq!(workspace_root.unwrap().join("Cargo.toml").exists(), true);
    }
}
