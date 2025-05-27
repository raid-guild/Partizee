
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
        let compile_args: CompileArgs = CompileArgs {
            files: None,
            build_args: None,
            additional_args: None,
        };
        Self::new(compile_args)
    }
}

impl ProjectCompiler {
    /// create a new builder with default settings
    pub fn new(compile_args: CompileArgs) -> Self {
        let project_root: PathBuf = find_workspace_root().unwrap_or_else(|| env::current_dir().unwrap());
        // if files is not None, convert files to PathBuf
        Self {
            project_root,
            files: compile_args.files,
            build_args: compile_args.build_args,
            additional_args: compile_args.additional_args,
        }   
    }

    fn gather_build_args<'a>(&self, args: &'a mut Vec<String>) -> &'a mut Vec<String> {
        if !self.build_args.is_none() {
            args.extend(self.build_args.as_ref().unwrap().iter().map(|arg| arg.to_string()));
        }
        args
    }

    fn gather_additional_args<'a>(&self, args: &'a mut Vec<String>) -> &'a mut Vec<String> {
        if !self.additional_args.is_none() {
            args.extend(self.additional_args.as_ref().unwrap().iter().map(|arg| arg.to_string()))
        }
        args
    }

    pub fn compile_contracts(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut output: Output;
        let mut args = vec![String::from("pbc"), String::from("build"), String::from("--release")];

        // gather build args and additional args
       self.gather_build_args(&mut args);
       self.gather_additional_args(&mut args);

        // if files is not None, compile the files
        if self.files.is_none() {
            // compile all contracts in the contracts directory add compiler args and build args
                output = Command::new("cargo")
                .args(&args)
                .output()
                .expect("Failed to compile contracts");
    
           
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                print_success_message(&output_str);
            } else {
                print_error_message("all contracts", String::from_utf8_lossy(&output.stderr).as_ref());
            }
             // else compile all contracts in the specified files
        } else {
            for file in self.files.as_ref().unwrap() {
                let mut new_args = args.clone();
                new_args.push(String::from("--manifest-path"));
                new_args.push(file.to_string());

                output = Command::new("cargo")
                    .args(&new_args)
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

