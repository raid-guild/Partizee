use std::{
    env,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Debug)]
pub struct ProjectCompiler {
    // the root of the project
    pub project_root: PathBuf,
    // extra files to include
    pub files: Vec<PathBuf>,
}

impl Default for ProjectCompiler {
    #[inline]
    fn default() -> Self{
        Self::new()
    }
}

impl ProjectCompiler {
    /// create a new builder with default settings
    pub fn new() -> Self {
        Self {
            project_root: PathBuf::new(),
            files: Vec::new(),
        }
    }

    /// Sets extra files to include, that are not necessarily in the project's source dir.
    #[inline]
    pub fn files(mut self, files: impl IntoIterator<Item = PathBuf>) -> Self {
            self.files.extend(files);
            self
    }

    pub fn compile_contracts(&self) -> Result<(), Box<dyn std::error::Error>> {
        let current_dir = env::current_dir()?;
        let contracts_dir = current_dir.join("contract");
        Ok(())
    }
}

pub fn execute(_config: ProjectCompiler) -> Result<(), Box<dyn std::error::Error>> {
    // Get current working directory
    let current_dir = env::current_dir()?;
    let contracts_dir = current_dir.join("contract");

    if !contracts_dir.exists() {
        return Err("No contracts directory found in current path".into());
    }

    let mut compiled_count = 0;
    compile_contracts_in_dir(&contracts_dir, &mut compiled_count)?;

    if compiled_count > 0 {
        println!("\n✨ Successfully compiled {} contract(s)", compiled_count);
    } else {
        println!("\n⚠️  No contracts found to compile");
    }

    Ok(())
}

fn compile_contracts_in_dir(dir: &Path, compiled_count: &mut i32) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            // Recursively compile contracts in subdirectories
            compile_contracts_in_dir(&path, compiled_count)?;
        } else if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            // Skip files that start with "mod" or "lib"
            if let Some(file_name) = path.file_name() {
                let file_name = file_name.to_string_lossy();
                if file_name.starts_with("mod.") || file_name.starts_with("lib.") {
                    continue;
                }
            }

            println!("\n🔨 Compiling contract: {}", path.display());
            
            // Build the contract using cargo partisia-contract build
            let output = Command::new("cargo")
                .args(["pbc", "build", "--manifest-path", path.to_str().unwrap()])
                .output()?;

            if output.status.success() {
                println!("✅ Successfully compiled {}", path.file_name().unwrap().to_string_lossy());
                *compiled_count += 1;
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                eprintln!("❌ Failed to compile {}: {}", path.file_name().unwrap().to_string_lossy(), error);
            }
        }
    }

    Ok(())
}
