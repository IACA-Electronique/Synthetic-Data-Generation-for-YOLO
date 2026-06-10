

pub trait DatasetConfig {
    fn from_base_dir(base_dir: String) -> Self;
    fn get_dataset_yaml_path(&self) -> String;
    fn get_images_dir_path(&self) -> String;
    fn get_images_train_dir_path(&self) -> String;
    fn get_images_val_dir_path(&self) -> String;
    fn get_images_test_dir_path(&self) -> String;
    fn get_labels_test_dir_path(&self) -> String;
    fn get_labels_train_dir_path(&self) -> String;
    fn get_labels_val_dir_path(&self) -> String;
}

/// Yolo dataset configuration for object detection with bounding boxes (OBB).
///
///  * `base_dir`: The base directory containing the dataset. Panic if empty.
///
/// Structure of the dataset:
/// ```text
/// base_dir/
/// ├── dataset.yaml
/// ├── images/
/// │   ├── train/
/// │   │   ├── img1.jpg
/// │   │   └── img2.jpg
/// │   ├── val/
/// │   │   ├── img101.jpg
/// │   │   └── img102.jpg
/// │   └── test/
/// │       ├── img201.jpg
/// │       └── img202.jpg
/// └── labels/
///     ├── train/
///     │   ├── img1.txt  # Contains 9-value lines for OBB
///     │   └── img2.txt
///     ├── val/
///     │   ├── img101.txt
///     │   └── img102.txt
///     └── test/
///         ├── img201.txt
///         └── img202.txt
/// ```
pub struct YOLOObbDatasetConfig {
    base_dir: String,
}

impl YOLOObbDatasetConfig {
    pub fn new(base_dir: String) -> Self {
        if base_dir.is_empty() {
            panic!("Base directory cannot be empty");
        }
        Self { base_dir }
    }
}

impl DatasetConfig for YOLOObbDatasetConfig {
    fn from_base_dir(base_dir: String) -> Self {
        Self::new(base_dir)
    }

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