use synthetic_data_generator_for_yolo::infrastructure::filesystem::MockFileSystem;
use synthetic_data_generator_for_yolo::models::dataset_config::MockDatasetConfig;
use synthetic_data_generator_for_yolo::services::dataset_directory_structure_generator::{
    DatasetDirectoryStructureGenerator, DatasetDirectoryStructureGeneratorImpl,
};

fn setup_full_config_mock() -> MockDatasetConfig {
    let mut mock = MockDatasetConfig::new();
    mock.expect_get_base_dir().returning(|| "/dataset".to_string());
    mock.expect_get_images_dir_path().returning(|| "/dataset/images".to_string());
    mock.expect_get_labels_dir_path().returning(|| "/dataset/labels".to_string());
    mock.expect_get_images_train_dir_path().returning(|| "/dataset/images/train".to_string());
    mock.expect_get_images_val_dir_path().returning(|| "/dataset/images/val".to_string());
    mock.expect_get_images_test_dir_path().returning(|| "/dataset/images/test".to_string());
    mock.expect_get_labels_train_dir_path().returning(|| "/dataset/labels/train".to_string());
    mock.expect_get_labels_val_dir_path().returning(|| "/dataset/labels/val".to_string());
    mock.expect_get_labels_test_dir_path().returning(|| "/dataset/labels/test".to_string());
    mock
}

#[test]
fn test_generate_structure_creates_all_directories() {
    let mock_config = setup_full_config_mock();

    let mut mock_fs = MockFileSystem::new();
    for path in [
        "/dataset",
        "/dataset/images",
        "/dataset/labels",
        "/dataset/images/train",
        "/dataset/images/val",
        "/dataset/images/test",
        "/dataset/labels/train",
        "/dataset/labels/val",
        "/dataset/labels/test",
    ] {
        mock_fs
            .expect_create_dir()
            .with(mockall::predicate::eq(path))
            .times(1)
            .returning(|_| Ok(()));
    }

    let generator = DatasetDirectoryStructureGeneratorImpl::new(&mock_config, &mock_fs);
    assert!(generator.generate_structure().is_ok());
}

#[test]
fn test_generate_structure_returns_error_when_base_dir_creation_fails() {
    let mut mock_config = MockDatasetConfig::new();
    mock_config.expect_get_base_dir().returning(|| "/dataset".to_string());

    let mut mock_fs = MockFileSystem::new();
    mock_fs
        .expect_create_dir()
        .with(mockall::predicate::eq("/dataset"))
        .times(1)
        .returning(|_| Err("permission denied".to_string()));

    let generator = DatasetDirectoryStructureGeneratorImpl::new(&mock_config, &mock_fs);
    let result = generator.generate_structure();

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Failed to create base directory: permission denied");
}

#[test]
fn test_generate_structure_returns_error_when_images_dir_creation_fails() {
    let mut mock_config = MockDatasetConfig::new();
    mock_config.expect_get_base_dir().returning(|| "/dataset".to_string());
    mock_config.expect_get_images_dir_path().returning(|| "/dataset/images".to_string());

    let mut mock_fs = MockFileSystem::new();
    mock_fs
        .expect_create_dir()
        .with(mockall::predicate::eq("/dataset"))
        .times(1)
        .returning(|_| Ok(()));
    mock_fs
        .expect_create_dir()
        .with(mockall::predicate::eq("/dataset/images"))
        .times(1)
        .returning(|_| Err("disk full".to_string()));

    let generator = DatasetDirectoryStructureGeneratorImpl::new(&mock_config, &mock_fs);
    let result = generator.generate_structure();

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Failed to create images directory: disk full");
}

#[test]
fn test_generate_structure_returns_error_when_last_dir_creation_fails() {
    let mut mock_config = MockDatasetConfig::new();
    mock_config.expect_get_base_dir().returning(|| "/dataset".to_string());
    mock_config.expect_get_images_dir_path().returning(|| "/dataset/images".to_string());
    mock_config.expect_get_labels_dir_path().returning(|| "/dataset/labels".to_string());
    mock_config.expect_get_images_train_dir_path().returning(|| "/dataset/images/train".to_string());
    mock_config.expect_get_images_val_dir_path().returning(|| "/dataset/images/val".to_string());
    mock_config.expect_get_images_test_dir_path().returning(|| "/dataset/images/test".to_string());
    mock_config.expect_get_labels_train_dir_path().returning(|| "/dataset/labels/train".to_string());
    mock_config.expect_get_labels_val_dir_path().returning(|| "/dataset/labels/val".to_string());
    mock_config.expect_get_labels_test_dir_path().returning(|| "/dataset/labels/test".to_string());

    let mut mock_fs = MockFileSystem::new();
    for path in [
        "/dataset",
        "/dataset/images",
        "/dataset/labels",
        "/dataset/images/train",
        "/dataset/images/val",
        "/dataset/images/test",
        "/dataset/labels/train",
        "/dataset/labels/val",
    ] {
        mock_fs
            .expect_create_dir()
            .with(mockall::predicate::eq(path))
            .times(1)
            .returning(|_| Ok(()));
    }
    mock_fs
        .expect_create_dir()
        .with(mockall::predicate::eq("/dataset/labels/test"))
        .times(1)
        .returning(|_| Err("no space left".to_string()));

    let generator = DatasetDirectoryStructureGeneratorImpl::new(&mock_config, &mock_fs);
    let result = generator.generate_structure();

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Failed to create labels test directory: no space left");
}
