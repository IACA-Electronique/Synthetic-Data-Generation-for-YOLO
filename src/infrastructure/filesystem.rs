use mockall::automock;
use std::fs;

#[automock]
pub trait FileSystem {
    fn list_files(&self, directory: &str) -> Result<Vec<String>, String>;
    fn list_subdirectories(&self, directory: &str) -> Result<Vec<String>, String>;
    fn is_dir(&self, directory: &str) -> bool;
    fn write_text(&self, output_path: &str, content: &str) -> Result<(), String>;
    fn create_dir(&self, directory: &str) -> Result<(), String>;
    fn get_image_size(&self, image_path: &str) -> Result<(u32, u32), String>;
}

#[derive(Default)]
pub struct SimpleFileSystem {}

impl SimpleFileSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl FileSystem for SimpleFileSystem {
    fn list_files(&self, directory: &str) -> Result<Vec<String>, String> {
        let entries = fs::read_dir(directory).map_err(|e| e.to_string())?;
        let files = entries
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    e.path()
                        .to_str()
                        .map(|s| s.to_string())
                        .filter(|_| e.path().is_file())
                })
            })
            .collect();
        Ok(files)
    }

    fn list_subdirectories(&self, directory: &str) -> Result<Vec<String>, String> {
        let entries = fs::read_dir(directory).map_err(|e| e.to_string())?;
        let subdirs = entries
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    e.path()
                        .to_str()
                        .map(|s| s.to_string())
                        .filter(|_| e.path().is_dir())
                })
            })
            .collect();
        Ok(subdirs)
    }

    fn is_dir(&self, directory: &str) -> bool {
        fs::metadata(directory).map(|m| m.is_dir()).unwrap_or(false)
    }

    fn write_text(&self, output_path: &str, content: &str) -> Result<(), String> {
        fs::write(output_path, content).map_err(|e| e.to_string())
    }

    fn create_dir(&self, directory: &str) -> Result<(), String> {
        fs::create_dir(directory).map_err(|e| e.to_string())
    }

    fn get_image_size(&self, image_path: &str) -> Result<(u32, u32), String> {
        let image = image::open(image_path).map_err(|e| e.to_string())?;
        Ok((image.width(), image.height()))
    }
}