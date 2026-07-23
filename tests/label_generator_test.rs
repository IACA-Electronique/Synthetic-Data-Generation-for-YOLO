use std::f32::consts::PI;

use mockall::predicate;
use synthetic_data_generator_for_yolo::infrastructure::filesystem::MockFileSystem;
use synthetic_data_generator_for_yolo::models::image_recipe::{ImageRecipe, PrintableElementRecipe};
use synthetic_data_generator_for_yolo::services::label_generator::{LabelGenerator, ObbYoloV11LabelGenerator};

fn make_recipe(name: &str, objects: Vec<PrintableElementRecipe>) -> ImageRecipe {
    let mut recipe = ImageRecipe::new();
    recipe.name = name.to_string();
    recipe.object = objects;
    recipe.width = 1;
    recipe.height = 1;
    recipe
}

#[test]
fn test_empty_recipe_writes_empty_label_file() {
    let mut mock_fs = MockFileSystem::new();
    mock_fs
        .expect_write_text()
        .with(predicate::eq("output/img.txt"), predicate::eq(""))
        .times(1)
        .returning(|_, _| Ok(()));

    let generator = ObbYoloV11LabelGenerator::new(&mock_fs);
    let result = generator.generate_one(make_recipe("img", vec![]), "output".to_string());

    assert!(result.is_ok());
}

#[test]
fn test_single_object_no_rotation_writes_correct_line() {

    let mut mock_fs = MockFileSystem::new();
    mock_fs
        .expect_write_text()
        .with(predicate::eq("out/scene.txt"), predicate::always())
        .times(1)
        .returning(|_, _| Ok(()));

    let object = PrintableElementRecipe::new("obj.png".to_string(), 0, 200, 200, 0.0, 10, 10);
    let generator = ObbYoloV11LabelGenerator::new(&mock_fs);
    let result = generator.generate_one(make_recipe("scene", vec![object]), "out".to_string());

    assert!(result.is_ok());
}

#[test]
fn test_single_object_half_turn_writes_correct_line() {

    let mut mock_fs = MockFileSystem::new();
    mock_fs
        .expect_write_text()
        .with(predicate::eq("labels/frame.txt"), predicate::always())
        .times(1)
        .returning(|_, _| Ok(()));

    let object = PrintableElementRecipe::new("obj.png".to_string(), 1, 200, 200, PI, 100, 100);
    let generator = ObbYoloV11LabelGenerator::new(&mock_fs);
    let result = generator.generate_one(make_recipe("frame", vec![object]), "labels".to_string());

    assert!(result.is_ok());
}

#[test]
fn test_multiple_objects_writes_one_line_per_object() {
    let mut mock_fs = MockFileSystem::new();
    mock_fs
        .expect_write_text()
        .with(predicate::eq("out/multi.txt"), predicate::always())
        .times(1)
        .returning(|_, _| Ok(()));

    let objects = vec![
        PrintableElementRecipe::new("a.png".to_string(), 0, 200, 200, 0.0, 10, 10),
        PrintableElementRecipe::new("b.png".to_string(), 2, 200, 200, PI, 100, 100),
    ];
    let generator = ObbYoloV11LabelGenerator::new(&mock_fs);
    let result = generator.generate_one(make_recipe("multi", objects), "out".to_string());

    assert!(result.is_ok());
}

#[test]
fn test_output_path_uses_recipe_name_and_output_dir() {
    let mut mock_fs = MockFileSystem::new();
    mock_fs
        .expect_write_text()
        .with(
            predicate::eq("/data/train/labels/my_image.txt"),
            predicate::always(),
        )
        .times(1)
        .returning(|_, _| Ok(()));

    let generator = ObbYoloV11LabelGenerator::new(&mock_fs);
    let result = generator.generate_one(
        make_recipe("my_image", vec![]),
        "/data/train/labels".to_string(),
    );

    assert!(result.is_ok());
}

#[test]
fn test_filesystem_error_is_propagated() {
    let mut mock_fs = MockFileSystem::new();
    mock_fs
        .expect_write_text()
        .times(1)
        .returning(|_, _| Err("disk full".to_string()));

    let generator = ObbYoloV11LabelGenerator::new(&mock_fs);
    let result = generator.generate_one(make_recipe("img", vec![]), "out".to_string());

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("disk full"), "error should contain the fs error: {err}");
    assert!(err.contains("out/img.txt"), "error should contain the file path: {err}");
}
