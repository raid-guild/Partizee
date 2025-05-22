use std::{
    env,
    fs,
    path::{Path, PathBuf},
    fs::File,
    io::Write,
};
use tera::{Context, Tera};

#[derive(Debug)]
pub struct NewProject {
    pub dapp_name: String,
    pub output_dir: PathBuf,
    // the root of the project
    pub project_root: PathBuf,
    pub executable_root: PathBuf,
}

impl NewProject {
    pub fn new(dapp_name: String, output_dir: Option<String>) -> Self {
        let project_root = env::current_dir().unwrap();
        let output_dir = output_dir.unwrap_or_else(|| format!("{}/", dapp_name.clone()));
        let output_buf: PathBuf = output_dir.into();
        NewProject {
            dapp_name,
            output_dir: output_buf,
            project_root,
            executable_root: env::current_exe().unwrap().parent().unwrap().to_path_buf(),
        }
    }

    pub fn create_project_directory(&self) -> Result<(), Box<dyn std::error::Error>> {
        let base_buff = self.output_dir.join("packages");
        let rust_buff = base_buff.join("rust");
        let contracts_buff = rust_buff.join("contracts");
        let rust_test_buff = rust_buff.join("test");
        let rust_src_buff = rust_buff.join("src");
        let nodejs_buff = base_buff.join("nodejs");
        fs::create_dir_all(&base_buff)?;
        fs::create_dir_all(&rust_buff)?;
        fs::create_dir_all(&contracts_buff)?;
        fs::create_dir_all(&rust_test_buff)?;
        fs::create_dir_all(&rust_src_buff)?;
        fs::create_dir_all(&nodejs_buff)?;
        self.print_project_structure();
        Ok(())
    }

    /// @param src: the source directory starting from the executable root "packages/templates/*"
    /// @param dst: the destination directory starting from the project root "contract/"
    /// @param template_name: the name of the template to copy "README.md"
    pub fn copy_template(&self, src: Option<&Path>, dst: Option<&Path>, template_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        
        // if src arg is provided, use it, otherwise use the default path
        let source_path = src.unwrap_or_else(|| &self.executable_root);
        // if dst arg is provided, use it, otherwise use the default path
        let destination_path = dst.unwrap_or_else(|| &self.project_root);

        // clean up the template name to remove the .template extension if exists
        let clean_template_name: String = template_name.replace(".template", "");
        let templates_dir = source_path.join("templates");
        let mut tera: Tera = Tera::default();
        
        // Process template
        let mut context: Context = Context::new();
        context.insert("project_name", &self.dapp_name);
        
        let readme_template = fs::read_to_string(templates_dir.join(&clean_template_name))?;
        
        let rendered = tera.render_str(&readme_template, &context)?;
        
        // write the rendered template to the destination path
        fs::write(destination_path.join(&clean_template_name), rendered)?;
    
        Ok(())
    }

    pub fn copy_all_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.copy_template(Some(&PathBuf::from("templates/")), Some(&self.output_dir), "README.md")?;
        self.copy_template(Some(&PathBuf::from("templates/rust")), Some(&self.output_dir.join("packages/rust/contracts")), "counter.rs")?;
        self.copy_template(Some(&PathBuf::from("templates/rust/src")), Some(&self.output_dir.join("packages/rust/src")), "main.rs")?;
        self.copy_template(None, None, "Cargo.template.toml")?;
        Ok(())
    }

    pub fn create_new_project(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.create_project_directory()?;
        self.copy_all_files()?;
        Ok(())
    }

    pub fn print_project_structure(&self) {
        println!("🚀 Creating new Partisia dapp: {}", self.dapp_name);
        println!("📁 Project created at: {}", self.output_dir.display());
        println!("  └─ 📂 rust/  (Partisia smart contracts)");
        println!("  └─ 📂 nodejs/  (Web 2 components)");
        println!("✨ Project scaffolding complete!");
        println!("\n📝 Next steps:");
        println!("  1. cd {}", self.output_dir.display());
        println!("  2. Follow the setup instructions in contract/README.md and frontend/README.md");
    }

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

