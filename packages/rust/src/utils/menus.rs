use cliclack::{confirm, input, intro, outro};

pub struct ProjectConfig {
    pub name: String,
    pub output_dir: Option<String>,
}

pub struct CompileArgs {
    pub files: Option<Vec<String>>,
    pub build_args: Option<Vec<String>>,
    pub additional_args: Option<Vec<String>>,
}

pub fn new_project_menu() -> Result<ProjectConfig, Box<dyn std::error::Error>> {
    intro("Partizee - Create a new Partisia Blockchain project")?;

    let name: String = input("What is your project name?")
        .placeholder("my-dapp")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Project name cannot be empty")
            } else {
                Ok(())
            }
        })
        .interact()?;

    let use_custom_dir = confirm("Would you like to specify a custom output directory? (if No: we'll create the project in the current directory with the name already specified)")
        .initial_value(false)
        .interact()?;

    let output_dir = if use_custom_dir {
        Some(
            input("Enter output directory path")
                .placeholder(&name)
                .interact()?,
        )
    } else {
        None
    };

    outro("Project configuration complete!")?;

    Ok(ProjectConfig { name, output_dir })
}

pub fn compile_menu() -> Result<CompileArgs, Box<dyn std::error::Error>> {
    let use_file_menu = confirm("Would you like to specify which contracts to compile? (if No: we'll compile all contracts in the contracts directory with default settings)")
    .initial_value(false)
    .interact()?;

    let mut build_args_vec: Vec<String> = Vec::new();
    let mut additional_args_vec: Vec<String> = Vec::new();
    let mut files_vec: Vec<String> = Vec::new();
    
    if use_file_menu {
        loop {
            let file_to_compile: String = input("Enter the path to a Cargo.toml of the contract to compile")
                .placeholder("contracts/counter/Cargo.toml")
                .validate(|input: &String| {
                    let path = std::path::Path::new(input);
                    if !path.exists() {
                        Err("File does not exist")
                    } else if !path.is_file() {
                        Err("Path is not a file")
                    } else if path.file_name().map_or(true, |name| name != "Cargo.toml") {
                        Err("File must be named Cargo.toml")
                    } else {
                        // Optionally, check for [package] section
                        let content = std::fs::read_to_string(path);
                        if let Ok(content) = content {
                            if content.contains("[package]") {
                                Ok(())
                            } else {
                                Err("Cargo.toml does not contain a [package] section")
                            }
                        } else {
                            Err("Could not read file")
                        }
                    }
                })
                .interact()?;
            if !file_to_compile.trim().is_empty() {
                files_vec.push(file_to_compile);
            }
            let another: bool = confirm("Enter another Contract?")
                .initial_value(false)
                .interact()?;
            if !another {
                break;
            }
        }
    }

    let add_build_args = confirm(
        "Would you like to specify Build arguments? \n 
    (if No: we'll compile all contracts in the contracts directory with default settings) \n
    Options: \n
    -r, --release                        Build artifacts in release mode, with optimizations \n
    -n, --no-abi                         Skip generating .abi file \n
    -q, --quiet                          No messages printed to stdout \n
    -w, --no-wasm-strip                  Do not remove custom sections from the WASM-file (will produce a much larger file). \n
    -z, --no-zk                          Only compile the public part of the contract. Skips compilation of ZK computation. \n
    --disable-git-fetch-with-cli     Uses cargo's built-in git library to fetch dependencies instead of the git executable \n
    --workspace                      Build all packages in the workspace \n
    --coverage                       Compile an instrumented binary for the smart contract. This enables generation of coverage files. \n
    -p, --package <PACKAGE>          Build only the specified packages \n
    -h, --help                       Print help \n
",
    )
    .initial_value(false)
    .interact()?;

    
        const AVAILABLE_FLAGS: &[&'static str; 17] = &["-r", "--release", "-n", "--no-abi", "-q", "--quiet", "-w", "--no-wasm-strip", "-z", "--no-zk", "--disable-git-fetch-with-cli", "--workspace", "--coverage", "-p", "--package", "-h", "--help"];

        if add_build_args {
            loop {
                let input_arg: String = input("Enter build argument ")
                    .placeholder("--help")
                    .validate(|input: &String| {
                        let input_arg = input.trim().to_string();
                        if AVAILABLE_FLAGS.iter().any(|flag| input_arg.contains(flag)) {
                            Ok(())
                        } else {
                            Err("Invalid build argument")
                        }
                    })
                    .interact()?;

                if !input_arg.trim().is_empty() {
                    build_args_vec.push(input_arg);
                }
                let another: bool = confirm("Enter another argument?")
                    .initial_value(false)
                    .interact()?;

                
                println!("build args: {:#?}", &build_args_vec);
                if !another {
                    break;
                }
            }
        }

        let add_additional_args = confirm(
            "Would you like to specify additional arguments? \n
     (if No: we'll compile all contracts in the contracts directory with default compiler arguments)",
        )
        .initial_value(false)
        .interact()?;

        if add_additional_args {
            loop {
                let additional_arg: String = input("Enter compiler argument:
                 (see https://partisiablockchain.gitlab.io/documentation/smart-contracts/smart-contract-tools-overview.html#command-line-tools for more info)")
                .placeholder("pbc cli arg")
                .interact()?;
                if !additional_arg.trim().is_empty() {
                    additional_args_vec.push(additional_arg);
                }
                let another: bool = confirm("Enter another argument? (y/n)")
                    .initial_value(false)
                    .interact()?;
                if !another {
                    break;
                }
            }
        }
        
    let files: Option<Vec<String>> = if files_vec.len() > 0 {
        Some(files_vec)
    } else {
        None
    };

    let build_args: Option<Vec<String>> = if build_args_vec.len() > 0 {
        Some(build_args_vec)
    } else {
        None
    };

    let additional_args: Option<Vec<String>> = if additional_args_vec.len() > 0 {
        Some(additional_args_vec)
    } else {
        None
    };

    Ok(CompileArgs {
        files,
        build_args,
        additional_args,
    })
}
