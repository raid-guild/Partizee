
use crate::utils::menus::{compile_menu, CompileArgs};
use crate::utils::utils::find_workspace_root;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

#[derive(Debug)]
pub struct ProjectCompiler {
    // the root of the project
    pub project_root: PathBuf,
    // extra files to include
    pub files: Option<Vec<String>>,
    pub build_args: Option<Vec<String>>,
    pub additional_args: Option<Vec<String>>,
}

impl Default for ProjectCompiler {
    #[inline]
    fn default() -> Self{
        let args: CompileArgs = CompileArgs {
            files: None,
            build_args: None,
            additional_args: None,
        };
        Self::new(args)
    }
}

impl ProjectCompiler {
    /// create a new builder with default settings
    pub fn new(args: CompileArgs) -> Self {
        let project_root: PathBuf = find_workspace_root().unwrap_or_else(|| env::current_dir().unwrap());
        // if files is not None, convert files to PathBuf
      
        Self {
            project_root,
            files: args.files,
            build_args: args.build_args,
            additional_args: args.additional_args,
        }   
    }

    pub fn compile_contracts(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut output: Output;
        let mut args = vec!["pbc", "build", "--release"];

        // gather build args and additional args
        if !self.build_args.is_none() {
            args.extend(self.build_args.as_ref().unwrap().iter().map(|arg| arg.as_str()));
        }
        if !self.additional_args.is_none() {
            args.extend(self.additional_args.as_ref().unwrap().iter().map(|arg| arg.as_str()));
        }

        // if files is not None, compile the files
        if self.files.is_none() {
            // compile all contracts in the contracts directory add compiler args and build args
                output = Command::new("cargo")
                .args(&args)
                .output()?;
    
            // else compile all contracts in the specified files
            if output.status.success() {
                print_success_message("all contracts");
            } else {
                print_error_message("all contracts", String::from_utf8_lossy(&output.stderr).as_ref());
            }
        } else {
            for file in self.files.as_ref().unwrap() {
                output = Command::new("cargo")
                    .args(&args)
                    .output()?;
                if output.status.success() {
                    print_success_message(file);
                } else {
                    print_error_message(file, String::from_utf8_lossy(&output.stderr).as_ref());
                }
            }
        }
        Ok(())
    }
}

pub fn print_success_message(file: &str) {
    println!("✅ Successfully compiled {}", file);
}

pub fn print_error_message(file: &str, error: &str) {
    eprintln!("❌ Failed to compile {}: {}", file, error);
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

