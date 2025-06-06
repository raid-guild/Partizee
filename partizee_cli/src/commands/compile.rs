use std::process::{Command, Output};
use std::path::PathBuf;

#[derive(Debug)]
pub struct ProjectCompiler {
    // extra files to include
    pub files: Option<Vec<String>>,
    pub path: Option<String>,
    pub build_args: Option<Vec<String>>,
    pub additional_args: Option<Vec<String>>,
}

impl Default for ProjectCompiler {
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
    /// create a new builder with default settings
    pub fn new(compile_args: ProjectCompiler) -> Self {
        // if files is not None, convert files to PathBuf
        Self {
            files: compile_args.files,
            path: compile_args.path,
            build_args: compile_args.build_args,
            additional_args: compile_args.additional_args,
        }
    }

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
            println!("PATH CHANGED TO: {}", current_path.to_str().unwrap());

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

fn extend_args<'a>(
    base_args: &'a mut Vec<String>,
    new_args: Option<&Vec<String>>,
) -> &'a mut Vec<String> {
    if new_args.is_some() {
        base_args.extend(new_args.unwrap().iter().map(|arg| arg.to_string()));
    }
    base_args
}

pub fn print_success_message(file: &str) {
    println!("✅ Successfully compiled {}", file);
}

pub fn print_error_message(file: &str, error: &str) {
    eprintln!("❌ Failed to compile {}: {}", file, error);
}
