use std::{
    env, fs,
    path::{Path, PathBuf},
};
use tera::{Context, Tera};
use walkdir::WalkDir;

use crate::utils::menus::{new_project_menu, ProjectConfig};
use crate::utils::utils::COPIABLE_EXTENSIONS;

#[derive(Debug)]
pub struct NewProject {
    pub dapp_name: String,
    pub output_dir: PathBuf,
    // the root of the project
    pub project_root: PathBuf,
    pub executable_root: PathBuf,
}

impl NewProject {
    pub fn new(dapp_name: Option<String>, output_dir: Option<String>) -> Self {
        // Get project config from CLI or menu
        let config: ProjectConfig = dapp_name
            .map(|name| ProjectConfig { name, output_dir })
            .unwrap_or_else(|| new_project_menu().unwrap());

        // install project in current directory
        let project_root = env::current_dir().unwrap();

        // if output_dir is provided, use it, otherwise use the project name
        let output_dir = config
            .output_dir
            .unwrap_or_else(|| format!("{}/", config.name.clone()))
            .into();

        NewProject {
            dapp_name: config.name,
            output_dir,
            project_root,
            executable_root: env::current_dir().unwrap(),
        }
    }

    pub fn create_project_directory(&self) -> Result<(), Box<dyn std::error::Error>> {
        let templates_dir = self.executable_root.join("templates");
        for entry in WalkDir::new(&templates_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_dir())
        {
            // Get the relative path from templates_dir
            let rel_path = entry.path().strip_prefix(&templates_dir)?;
            // Skip the root itself
            if rel_path.as_os_str().is_empty() {
                continue;
            }
            // Create the corresponding directory in the new project root
            let new_dir = self.output_dir.join(rel_path);
            fs::create_dir_all(&new_dir)?;
        }

        self.print_project_structure();

        Ok(())
    }

    /// @param src: the source directory starting from the executable root "packages/templates/*"
    /// @param dst: the destination directory starting from the project root "contract/"
    /// @param template_name: the name of the template to copy "README.md"
    pub fn copy_template(
        &self,
        src: Option<&Path>,
        dst: Option<&Path>,
        template_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // if src arg is provided, use it, otherwise use the default path
        let source_path = src.unwrap_or_else(|| &self.executable_root);
        // if dst arg is provided, use it, otherwise use the default path
        let destination_path = dst.unwrap_or_else(|| &self.project_root);

        // clean up the template name to remove the .template extension if exists
        let clean_template_name: String = template_name.replace(".template", "");
        // check if the file is a copiable extension otherwise use tera
        if COPIABLE_EXTENSIONS
            .iter()
            .any(|ext| clean_template_name.ends_with(ext))
        {
            // copy with fs
            fs::copy(
                source_path.join(&template_name),
                destination_path.join(&clean_template_name),
            )?;
        } else {
            // copy with tera
            let mut tera: Tera = Tera::default();

            // Process template
            let mut context: Context = Context::new();
            context.insert("project_name", &self.dapp_name);

            let base_template = fs::read_to_string(source_path.join(&template_name))?;

            let rendered = tera.render_str(&base_template, &context)?;

            // write the rendered template to the destination path
            fs::write(destination_path.join(&clean_template_name), rendered)?;
        }

        Ok(())
    }

    pub fn copy_all_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        let templates_dir: PathBuf = self.executable_root.join("templates");
        for entry in WalkDir::new(&templates_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let rel_path = entry.path().strip_prefix(&templates_dir)?;
            let dest_path = self.output_dir.join(rel_path);

            // Ensure parent directories exist
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Use your template logic if needed, or just copy the file
            self.copy_template(
                Some(entry.path().parent().unwrap()),
                Some(dest_path.parent().unwrap()),
                entry
                    .file_name()
                    .to_str()
                    .ok_or("Invalid UTF-8 in file name")?,
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
        println!("🚀 Creating new Partisia dapp: {}", self.dapp_name);
        println!("📁 Project created at: {}", self.output_dir.display());
        println!("  └─ 📂 rust/  (Partisia smart contracts)");
        println!("  └─ 📂 frontend/  (Web 2 components)");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_dir_contents() {
        copy_dir_contents(
            &Path::new("/home/someguy/code/Partizee/packages/templates"),
            &Path::new("/home/someguy/code/Partizee/contract"),
        )
        .unwrap();
    }
}
