use synthetic_data_generator_for_yolo::infrastructure::filesystem::MockFileSystem;
use synthetic_data_generator_for_yolo::services::dataset_yaml_generator::{DatasetYamlGenerator, DatasetYamlGeneratorImpl};

#[test]
fn test_generate_yaml_returns_correct_filepath() {
    let mut mock_fs = MockFileSystem::new();
    mock_fs
        .expect_write_text()
        .with(
            mockall::predicate::eq("/output/dataset.yaml"),
            mockall::predicate::always(),
        )
        .times(1)
        .returning(|_, _| Ok(()));

    let generator = DatasetYamlGeneratorImpl::new("/output".to_string(), &mock_fs);
    let result = generator.generate_yaml();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "/output/dataset.yaml");
}

#[test]
fn test_generate_yaml_content_has_correct_paths() {
    let mut mock_fs = MockFileSystem::new();
    mock_fs
        .expect_write_text()
        .withf(|path, content| {
            path == "/data/dataset.yaml"
                && content.contains("path: /data")
                && content.contains("train: /data/train")
                && content.contains("test: /data/test")
                && content.contains("val: /data/val")
        })
        .times(1)
        .returning(|_, _| Ok(()));

    let generator = DatasetYamlGeneratorImpl::new("/data".to_string(), &mock_fs);
    let result = generator.generate_yaml();

    assert!(result.is_ok());
}

#[test]
fn test_generate_yaml_propagates_filesystem_error() {
    let mut mock_fs = MockFileSystem::new();
    mock_fs
        .expect_write_text()
        .times(1)
        .returning(|_, _| Err("permission denied".to_string()));

    let generator = DatasetYamlGeneratorImpl::new("/output".to_string(), &mock_fs);
    let result = generator.generate_yaml();

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "permission denied");
}
