use synthetic_data_generator_for_yolo::models::dataset_config::{DatasetConfig, YOLOObbDatasetConfig};

#[test]
fn test_new_with_valid_base_dir_returns_correct_paths() {
    let config = YOLOObbDatasetConfig::new("/data".to_string());

    assert_eq!(config.get_dataset_yaml_path(), "/data/dataset.yaml");
    assert_eq!(config.get_images_dir_path(), "/data/images");
    assert_eq!(config.get_images_train_dir_path(), "/data/images/train");
    assert_eq!(config.get_images_val_dir_path(), "/data/images/val");
    assert_eq!(config.get_images_test_dir_path(), "/data/images/test");
    assert_eq!(config.get_labels_train_dir_path(), "/data/labels/train");
    assert_eq!(config.get_labels_val_dir_path(), "/data/labels/val");
    assert_eq!(config.get_labels_test_dir_path(), "/data/labels/test");
}

#[test]
#[should_panic(expected = "Base directory cannot be empty")]
fn test_new_with_empty_base_dir_panics() {
    YOLOObbDatasetConfig::new("".to_string());
}
