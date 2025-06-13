use std::path::PathBuf;
use std::process::{Command, Output};

/// Configuration for compiling Partisia Blockchain contracts
/// 
/// # Fields
/// * `files` - Optional list of specific contract files to compile
/// * `path` - Optional path to workspace directory
/// * `build_args` - Optional build arguments for cargo
/// * `additional_args` - Optional additional arguments for the compiler
#[derive(Debug)]
pub struct ProjectCompiler {
    // extra files to include
    pub files: Option<Vec<String>>,
    pub path: Option<String>,
    pub build_args: Option<Vec<String>>,
    pub additional_args: Option<Vec<String>>,
}

impl Default for ProjectCompiler {
    /// Creates a new ProjectCompiler with default settings
    /// All fields are set to None
    #[inline]
    fn default() -> Self {
        let compile_args: ProjectCompiler = ProjectCompiler {
            files: None,
            path: None,
            build_args: None,
            additional_args: None,
        };
        Self::new(compile_args)
    }
}

impl ProjectCompiler {
    /// Creates a new ProjectCompiler with the specified settings
    /// 
    /// # Arguments
    /// * `compile_args` - Configuration for the compiler
    /// 
    /// # Returns
    /// * `ProjectCompiler` - New compiler instance
    pub fn new(compile_args: ProjectCompiler) -> Self {
        // if files is not None, convert files to PathBuf
        Self {
            files: compile_args.files,
            path: compile_args.path,
            build_args: compile_args.build_args,
            additional_args: compile_args.additional_args,
        }
    }

    /// Compiles the specified contracts using cargo
    /// 
    /// If no specific files are provided, compiles all contracts in the workspace
    /// Uses release mode by default and applies any specified build/additional arguments
    /// 
    /// # Returns
    /// * `Result<(), Box<dyn std::error::Error>>` - Ok if compilation succeeds, Error otherwise
    pub fn compile_contracts(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut output: Output;
        let mut args = vec![
            String::from("pbc"),
            String::from("build"),
            String::from("--release"),
        ];

        // gather build args and additional args
        extend_args(&mut args, self.build_args.as_ref());
        extend_args(&mut args, self.additional_args.as_ref());

        if self.path.is_some() {
            let path_arg: String = self.path.as_ref().unwrap().to_string();
            if !PathBuf::from(&path_arg).is_dir() {
                return Err("Path is not a directory".into());
            }
            // get absolute path
            let current_path = PathBuf::from(&path_arg).canonicalize()?;
            std::env::set_current_dir(&current_path)?;
            println!("PATH CHANGED TO: {}", current_path.display());
        }

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
                print_error_message(
                    "all contracts",
                    String::from_utf8_lossy(&output.stderr).as_ref(),
                );
            }
            // else compile all contracts in the specified files
        } else {
            for file in self.files.as_ref().unwrap() {
                let mut new_args = args.clone();
                new_args.push(String::from("--manifest-path"));
                new_args.push(file.to_string());
                output = Command::new("cargo").args(&new_args).output()?;

                if output.status.success() {
                    print_success_message(file);
                } else {
                    println!("{:#?}", &output);
                    print_error_message(file, String::from_utf8_lossy(&output.stderr).as_ref());
                }
            }
        }
        Ok(())
    }
}

/// Extends a vector of arguments with optional additional arguments
/// 
/// # Arguments
/// * `base_args` - Vector to extend
/// * `new_args` - Optional vector of arguments to add
/// 
/// # Returns
/// * `&mut Vec<String>` - Reference to the extended vector
fn extend_args<'a>(
    base_args: &'a mut Vec<String>,
    new_args: Option<&Vec<String>>,
) -> &'a mut Vec<String> {
    if new_args.is_some() {
        base_args.extend(new_args.unwrap().iter().map(|arg| arg.to_string()));
    }
    base_args
}

/// Prints a success message for a compiled file
/// 
/// # Arguments
/// * `file` - Name of the successfully compiled file
pub fn print_success_message(file: &str) {
    println!("✅ Successfully compiled {}", file);
}

/// Prints an error message for a failed compilation
/// 
/// # Arguments
/// * `file` - Name of the file that failed to compile
/// * `error` - Error message from the compiler
pub fn print_error_message(file: &str, error: &str) {
    eprintln!("❌ Failed to compile {}: {}", file, error);
}
