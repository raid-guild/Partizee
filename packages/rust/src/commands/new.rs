use std::env;
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Write;

pub struct NewConfig {
    pub dapp_name: String,
    pub output_dir: String,
}

impl NewConfig {
    pub fn new(dapp_name: String, output_dir: Option<String>) -> Self {
        let output_dir = output_dir.unwrap_or_else(|| dapp_name.clone());
        NewConfig {
            dapp_name,
            output_dir,
        }
    }
}

pub fn execute(config: NewConfig) -> Result<(), Box<dyn std::error::Error>> {
    // Get current working directory
    let current_dir = env::current_dir()?;
    
    // Create project directory in current working directory
    let project_dir = current_dir.join(&config.output_dir);
    fs::create_dir_all(&project_dir)?;

    // Get templates directory relative to executable
    let exe_path = env::current_exe()?;
    let exe_dir = exe_path.parent().ok_or("Could not get executable directory")?;
    let templates_dir = exe_dir.join("templates");

    // Copy smart contract template
    let contract_dir = project_dir.join("contracts");
    fs::create_dir_all(&contract_dir)?;
    copy_dir_contents(&templates_dir.join("rust"), &contract_dir)?;

    // Copy frontend template
    let frontend_dir = project_dir.join("frontend");
    fs::create_dir_all(&frontend_dir)?;
    copy_dir_contents(&templates_dir.join("nodejs"), &frontend_dir)?;

    println!("🚀 Creating new Partisia dapp: {}", config.dapp_name);
    println!("📁 Project created at: {}", project_dir.display());
    println!("  └─ 📂 contracts/  (Partisia smart contracts)");
    println!("  └─ 📂 frontend/  (Web frontend)");
    println!("✨ Project scaffolding complete!");
    println!("\n📝 Next steps:");
    println!("  1. cd {}", config.output_dir);
    println!("  2. Follow the setup instructions in contract/README.md and frontend/README.md");

    Ok(())
}

fn copy_dir_contents(src: &Path, dst: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !src.is_dir() {
        return Err("Source path is not a directory".into());
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(path.file_name().ok_or("Invalid file name")?);

        if path.is_dir() {
            fs::create_dir_all(&dest_path)?;
            copy_dir_contents(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }

    Ok(())
}

fn copy_selected_contract_and_create_main(src: &Path, dst: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure the destination directory exists
    fs::create_dir_all(dst)?;

    // Copy the selected contract template file into the destination directory
    let contract_file_name = src.file_name().ok_or("Invalid contract template file name")?;
    let dest_contract_path = dst.join(contract_file_name);
    fs::copy(src, &dest_contract_path)?;

    // Create a src/ directory for the new project
    let src_dir = dst.join("src");
    fs::create_dir_all(&src_dir)?;

    // Create a main.rs file with placeholder content
    let main_rs_path = src_dir.join("main.rs");
    let mut main_rs_file = File::create(main_rs_path)?;
    writeln!(main_rs_file, "// main.rs placeholder\n\nfn main() {{\n    // TODO: Add your functions here\n}}")?;

    Ok(())
}
