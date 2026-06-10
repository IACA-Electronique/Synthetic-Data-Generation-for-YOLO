use crate::infrastructure::filesystem::FileSystem;
use crate::models::dataset_config::DatasetConfig;

pub trait DatasetYamlGenerator {
    fn generate_yaml(&self) -> Result<String, String>;
}

pub struct DatasetYamlGeneratorImpl<'a, FS: FileSystem, C: DatasetConfig>  {
    dataset_config: &'a C,
    filesystem: &'a FS,
}

impl<'a,FS: FileSystem,C: DatasetConfig>  DatasetYamlGeneratorImpl<'a, FS, C> {
    pub fn new(dataset_config: &'a C, filesystem: &'a FS) -> Self {
        Self { dataset_config, filesystem}
    }

    fn get_yaml_content(&self) -> String {
        let train_dir = self.dataset_config.get_dataset_yaml_train_path();
        let test_dir = self.dataset_config.get_dataset_yaml_test_path();
        let val_dir = self.dataset_config.get_dataset_yaml_val_path();

        format!("path: {}\ntrain: {}\ntest: {}\nval: {}", self.dataset_config.get_base_dir(), train_dir, test_dir, val_dir)
    }
}

impl<'a,FS: FileSystem,C: DatasetConfig>  DatasetYamlGenerator for DatasetYamlGeneratorImpl<'a, FS, C> {
    fn generate_yaml(&self) -> Result<String, String> {
        let yaml_content = self.get_yaml_content();

        let yaml_filepath = self.dataset_config.get_dataset_yaml_path();
        self.filesystem.write_text(&yaml_filepath, &yaml_content)?;

        Ok(yaml_filepath)
    }
}