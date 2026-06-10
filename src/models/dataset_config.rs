

pub trait DatasetConfig {
    fn get_dataset_yaml_path(&self) -> String;
    fn get_images_dir_path(&self) -> String;
    fn get_images_train_dir_path(&self) -> String;
    fn get_images_val_dir_path(&self) -> String;
    fn get_images_test_dir_path(&self) -> String;
    fn get_labels_test_dir_path(&self) -> String;
    fn get_labels_train_dir_path(&self) -> String;
    fn get_labels_val_dir_path(&self) -> String;
}

pub struct YOLOObbDatasetConfig {
    base_dir: String,
}

impl YOLOObbDatasetConfig {
    pub fn new(base_dir: String) -> Self {
        Self { base_dir }
    }
}

impl DatasetConfig for YOLOObbDatasetConfig {
    fn get_dataset_yaml_path(&self) -> String {
        format!("{}/dataset.yaml", self.base_dir)
    }

    fn get_images_dir_path(&self) -> String {
        format!("{}/images", self.base_dir)
    }

    fn get_images_train_dir_path(&self) -> String {
        format!("{}/images/train", self.base_dir)
    }

    fn get_images_val_dir_path(&self) -> String {
        format!("{}/images/val", self.base_dir)
    }

    fn get_images_test_dir_path(&self) -> String {
        format!("{}/images/test", self.base_dir)
    }

    fn get_labels_test_dir_path(&self) -> String {
        format!("{}/labels/test", self.base_dir)
    }

    fn get_labels_train_dir_path(&self) -> String {
        format!("{}/labels/train", self.base_dir)
    }

    fn get_labels_val_dir_path(&self) -> String {
        format!("{}/labels/val", self.base_dir)
    }
}