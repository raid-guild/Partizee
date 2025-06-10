use crate::utils::fs_nav::{find_dir};

use crate::utils::utils::TERA_EXTENSIONS;
use std::error::Error;
use std::{
    fs,
    path::{Path, PathBuf},
    env,
};
use tera::{Context, Tera};
use walkdir::WalkDir;
use rust_embed::Embed;

pub struct NewProject {
    pub dapp_name: String,
    pub output_dir: PathBuf,
    // the root of the project
    pub project_root: PathBuf,
}

pub struct ProjectConfig {
    pub name: String,
    pub output_dir: Option<String>,
}

#[derive(Embed)]
#[folder = "templates/"]
struct Templates;

impl NewProject {
    pub fn new(config: ProjectConfig) -> Result<NewProject, Box<dyn Error>> {
        // install project in current directory
        let project_root = env::current_dir().unwrap_or_else(|_| {
            panic!("Failed to find project root");
        });

        // if output_dir is provided, use it, otherwise use the project name
        let output_dir = config
            .output_dir
            .unwrap_or_else(|| format!("{}/", config.name.clone()))
            .into();

        Ok(NewProject {
            dapp_name: config.name,
            output_dir,
            project_root: project_root.clone(),
        })
    }

    pub fn create_project_directory(&self) -> Result<(), Box<dyn std::error::Error>> {
        for entry in Templates::iter()
        {
            let dir_path = entry.as_ref();
            if PathBuf::from(dir_path).is_dir() {
            // Create the corresponding directory in the new project root
            let new_dir = self.output_dir.join(dir_path);
            fs::create_dir_all(&new_dir)?;
            }
        }

        self.print_project_structure();

        Ok(())
    }

    /// @param src: the source directory starting from the executable root "packages/templates/*"
    /// @param dst: the destination directory starting from the project root "contract/"
    /// @param template_name: the name of the template to copy "README.md"
    pub fn copy_template(
        &self,
        src: &Path,
        dst: &Path,
        template_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let destination_path = dst;
        let template_content = Templates::get(src.to_str().unwrap_or_else(|| panic!("Invalid UTF-8 in file name"))).ok_or_else(|| "Template not found")?;
        // clean up the template name to remove the .template extension if exists
        let clean_template_name: String = template_name.replace(".template", "");
        // check if the file is a copiable extension otherwise use tera
        if TERA_EXTENSIONS
            .iter()
            .any(|ext| clean_template_name.ends_with(ext))
        {
                    // copy with tera
                    let mut tera: Tera = Tera::default();
                    tera.autoescape_on(vec![]); // Disable autoescaping for all files

                    // Process template
                    let mut context: Context = Context::new();
                    context.insert("project_name", &self.dapp_name);
        
                    let base_template = std::str::from_utf8(template_content.data.as_ref())?;
        
                    let rendered = tera.render_str(&base_template, &context)?;
        
                    // write the rendered template to the destination path
                    fs::write(destination_path.join(&clean_template_name), rendered)?;
        } else {
                // copy with fs
                fs::write(
                    destination_path.join(&clean_template_name),
                    template_content.data.as_ref(),
                )?;
        }

        Ok(())
    }

    pub fn copy_all_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        for entry in Templates::iter() {
            let path = entry.as_ref();
            
            let dest_path = self.output_dir.join(path);
            
            // Skip directories, just ensure they exist
            if path.ends_with('/') {
                fs::create_dir_all(&dest_path)?;
                continue;
            }

            // Ensure parent directory exists
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Get the file name for the template
            let file_name = Path::new(path)
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| format!("Invalid file name: {}", path))?;
            
            // Copy the file using template processing
            self.copy_template(
                Path::new(path),
                &dest_path.parent().unwrap(),
                file_name,
            )?;
        }
        Ok(())
    }

    pub fn create_new_project(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.create_project_directory()?;
        self.copy_all_files()?;
        Ok(())
    }

    pub fn print_project_structure(&self) {
        println!("\n \n \n \n");
        println!("📁 Project created at: {}", self.output_dir.display());
        println!("  └─ 📂 rust/  (Partisia smart contracts)");
        println!("  └─ 📂 frontend/  (Web 2 components)");
        println!("✨ Project scaffolding complete!");
        println!("\n📝 Next steps:");
        println!("  1. cd {}", self.output_dir.display());
        println!("  2. Follow the setup instructions in contract/README.md and frontend/README.md");
    }
}
