use crate::infrastructure::filesystem::FileSystem;
use crate::models::dataset_config::DatasetConfig;

pub trait DatasetDirectoryStructureGenerator {
    fn generate_structure(&self);
}

pub struct DatasetDirectoryStructureGeneratorImpl<'a, FS: FileSystem, C: DatasetConfig> {
    dataset_config: &'a C,
    filesystem: &'a FS,
}

impl<'a, FS: FileSystem, C: DatasetConfig> DatasetDirectoryStructureGeneratorImpl<'a, FS, C> {
    pub fn new(dataset_config: &'a C, filesystem: &'a FS) -> Self {
        Self { dataset_config, filesystem }
    }
}

impl<'a, FS: FileSystem, C: DatasetConfig> DatasetDirectoryStructureGenerator for DatasetDirectoryStructureGeneratorImpl<'a, FS, C> {
    fn generate_structure(&self) {
        self.filesystem.create_dir(&self.dataset_config.get_base_dir())
            .map_err(|e| format!("Failed to create base directory: {}", e)).unwrap();

        self.filesystem.create_dir(&self.dataset_config.get_images_dir_path())
            .map_err(|e| format!("Failed to create images directory: {}", e)).unwrap();

        self.filesystem.create_dir(&self.dataset_config.get_images_train_dir_path())
            .map_err(|e| format!("Failed to create images train directory: {}", e)).unwrap();

        self.filesystem.create_dir(&self.dataset_config.get_images_val_dir_path())
            .map_err(|e| format!("Failed to create images val directory: {}", e)).unwrap();

        self.filesystem.create_dir(&self.dataset_config.get_images_test_dir_path())
            .map_err(|e| format!("Failed to create images test directory: {}", e)).unwrap();

        self.filesystem.create_dir(&self.dataset_config.get_labels_train_dir_path())
            .map_err(|e| format!("Failed to create labels train directory: {}", e)).unwrap();

        self.filesystem.create_dir(&self.dataset_config.get_labels_val_dir_path())
            .map_err(|e| format!("Failed to create labels val directory: {}", e)).unwrap();

        self.filesystem.create_dir(&self.dataset_config.get_labels_test_dir_path())
            .map_err(|e| format!("Failed to create labels test directory: {}", e)).unwrap();
    }
}