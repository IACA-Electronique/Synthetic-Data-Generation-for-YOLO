use crate::infrastructure::filesystem::FileSystem;

pub trait DatasetYamlGenerator {
    fn generate_yaml(&self) -> Result<String, String>;
}

pub struct DatasetYamlGeneratorImpl<'a, FS: FileSystem>  {
    output_dir: String,
    filesystem: &'a FS,
}

impl<'a,FS: FileSystem>  DatasetYamlGeneratorImpl<'a, FS> {
    pub fn new(output_dir: String, filesystem: &'a FS) -> Self {
        Self { output_dir, filesystem}
    }

    fn get_yaml_content(&self) -> String {
        let train_dir = format!("{}/train", self.output_dir);
        let test_dir = format!("{}/test", self.output_dir);
        let val_dir = format!("{}/val", self.output_dir);

        let text = format!("path: {}\ntrain: {}\ntest: {}\nval: {}", self.output_dir, train_dir, test_dir, val_dir);
        return text;
    }
}

impl<'a,FS: FileSystem>  DatasetYamlGenerator for DatasetYamlGeneratorImpl<'a, FS> {
    fn generate_yaml(&self) -> Result<String, String> {
        let yaml_content = self.get_yaml_content();

        let yaml_filepath = format!("{}/dataset.yaml", self.output_dir);
        self.filesystem.write_text(&yaml_filepath, &yaml_content)?;

        Ok(yaml_filepath)
    }
}