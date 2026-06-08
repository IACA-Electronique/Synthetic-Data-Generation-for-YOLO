use mockall::automock;
use std::fs;

#[automock]
pub trait FileSystem {
    fn list_files(&self, directory: &str) -> Result<Vec<String>, String>;
    fn list_subdirectories(&self, directory: &str) -> Result<Vec<String>, String>;
}

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
}