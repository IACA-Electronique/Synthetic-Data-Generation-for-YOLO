use synthetic_data_generator_for_yolo::infrastructure::builders::editable_image_builder::EditableImageBuilder;
use synthetic_data_generator_for_yolo::infrastructure::editable_image::{EditableImage, MockEditableImage};
use synthetic_data_generator_for_yolo::services::image_generator::{ImageGenerator, ImageGeneratorImpl};
use synthetic_data_generator_for_yolo::models::image_recipe::{ImageRecipe, PrintableElementRecipe};

struct TestEditableImageBuilder;

impl EditableImageBuilder for TestEditableImageBuilder {
    type Image = MockEditableImage;

    fn build_from_nothing(width: u32, height: u32) -> MockEditableImage {
        MockEditableImage::from_nothing(width, height)
    }
}

#[test]
fn test_generate_empty_recipes() {
    let generator = ImageGeneratorImpl::<TestEditableImageBuilder>::new();
    let result = generator.generate(vec![], String::new());
    assert!(result.is_ok());
}

#[test]
fn test_generate_image_with_object_and_distraction() {
    let generator = ImageGeneratorImpl::<TestEditableImageBuilder>::new();

    let mut recipe = ImageRecipe::new();
    recipe.background_path = "bg.png".to_string();
    recipe.width = 640;
    recipe.height = 480;
    recipe.object = vec![
        PrintableElementRecipe::new("obj1.png".to_string(), 1, 1.0, 0.0, 10, 20),
    ];
    recipe.distraction = Some(vec![
        PrintableElementRecipe::new("dist1.png".to_string(), 2, 0.5, 45.0, 100, 200),
    ]);

    let ctx = MockEditableImage::from_nothing_context();
    ctx.expect()
        .with(mockall::predicate::eq(640u32), mockall::predicate::eq(480u32))
        .times(1)
        .returning(|_, _| {
            let mut mock_image = MockEditableImage::default();

            mock_image.expect_set_background_from_file()
                .with(mockall::predicate::eq("bg.png"))
                .times(1)
                .returning(|_| ());

            mock_image.expect_add_scalable_object_from_file()
                .with(
                    mockall::predicate::eq("obj1.png"),
                    mockall::predicate::eq(10u32),
                    mockall::predicate::eq(20u32),
                    mockall::predicate::eq(1.0f32),
                    mockall::predicate::eq(0.0f32),
                )
                .times(1)
                .returning(|_, _, _, _, _| ());

            mock_image.expect_add_scalable_object_from_file()
                .with(
                    mockall::predicate::eq("dist1.png"),
                    mockall::predicate::eq(100u32),
                    mockall::predicate::eq(200u32),
                    mockall::predicate::eq(0.5f32),
                    mockall::predicate::eq(45.0f32),
                )
                .times(1)
                .returning(|_, _, _, _, _| ());

            mock_image.expect_save()
                .with(mockall::predicate::str::starts_with("/out/test"))
                .times(1)
                .returning(|_| ());

            mock_image
        });

    let result = generator.generate(vec![recipe], "/out/test".to_string());
    assert!(result.is_ok());
}

#[test]
fn test_generate_image_no_distractions() {
    let generator = ImageGeneratorImpl::<TestEditableImageBuilder>::new();

    let mut recipe = ImageRecipe::new();
    recipe.background_path = "bg2.png".to_string();
    recipe.width = 320;
    recipe.height = 240;
    recipe.object = vec![
        PrintableElementRecipe::new("obj2.png".to_string(), 3, 2.0, 90.0, 30, 40),
    ];
    recipe.distraction = None;

    let ctx = MockEditableImage::from_nothing_context();
    ctx.expect()
        .with(mockall::predicate::eq(320u32), mockall::predicate::eq(240u32))
        .times(1)
        .returning(|_, _| {
            let mut mock_image = MockEditableImage::default();

            mock_image.expect_set_background_from_file()
                .with(mockall::predicate::eq("bg2.png"))
                .times(1)
                .returning(|_| ());

            mock_image.expect_add_scalable_object_from_file()
                .with(
                    mockall::predicate::eq("obj2.png"),
                    mockall::predicate::eq(30u32),
                    mockall::predicate::eq(40u32),
                    mockall::predicate::eq(2.0f32),
                    mockall::predicate::eq(90.0f32),
                )
                .times(1)
                .returning(|_, _, _, _, _| ());

            mock_image.expect_save()
                .with(mockall::predicate::str::starts_with("/out/test"))
                .times(1)
                .returning(|_| ());

            mock_image
        });

    let result = generator.generate(vec![recipe], "/out/test".to_string());
    assert!(result.is_ok());
}

#[test]
fn test_generate_one_adds_distractions_before_objects() {
    let generator = ImageGeneratorImpl::<TestEditableImageBuilder>::new();

    let mut recipe = ImageRecipe::new();
    recipe.background_path = "bg_order.png".to_string();
    recipe.width = 640;
    recipe.height = 480;
    recipe.object = vec![
        PrintableElementRecipe::new("obj_order.png".to_string(), 1, 1.0, 0.0, 10, 20),
    ];
    recipe.distraction = Some(vec![
        PrintableElementRecipe::new("dist_order.png".to_string(), 2, 0.5, 45.0, 100, 200),
    ]);

    let ctx = MockEditableImage::from_nothing_context();
    ctx.expect()
        .with(mockall::predicate::eq(640u32), mockall::predicate::eq(480u32))
        .times(1)
        .returning(|_, _| {
            let mut mock_image = MockEditableImage::default();

            mock_image.expect_set_background_from_file()
                .with(mockall::predicate::eq("bg_order.png"))
                .times(1)
                .returning(|_| ());

            // A single sequence forces the distraction call to be recorded
            // before the object call; mockall fails the test if the object is
            // added first.
            let mut seq = mockall::Sequence::new();

            mock_image.expect_add_scalable_object_from_file()
                .with(
                    mockall::predicate::eq("dist_order.png"),
                    mockall::predicate::eq(100u32),
                    mockall::predicate::eq(200u32),
                    mockall::predicate::eq(0.5f32),
                    mockall::predicate::eq(45.0f32),
                )
                .times(1)
                .in_sequence(&mut seq)
                .returning(|_, _, _, _, _| ());

            mock_image.expect_add_scalable_object_from_file()
                .with(
                    mockall::predicate::eq("obj_order.png"),
                    mockall::predicate::eq(10u32),
                    mockall::predicate::eq(20u32),
                    mockall::predicate::eq(1.0f32),
                    mockall::predicate::eq(0.0f32),
                )
                .times(1)
                .in_sequence(&mut seq)
                .returning(|_, _, _, _, _| ());

            mock_image.expect_save()
                .with(mockall::predicate::str::starts_with("/out/test"))
                .times(1)
                .returning(|_| ());

            mock_image
        });

    let result = generator.generate_one(recipe, "/out/test".to_string());
    assert!(result.is_ok());
}

#[test]
fn test_generate_multiple_recipes() {
    let generator = ImageGeneratorImpl::<TestEditableImageBuilder>::new();

    let mut recipe1 = ImageRecipe::new();
    recipe1.background_path = "bg_multi1.png".to_string();
    recipe1.width = 100;
    recipe1.height = 100;

    let mut recipe2 = ImageRecipe::new();
    recipe2.background_path = "bg_multi2.png".to_string();
    recipe2.width = 200;
    recipe2.height = 200;

    let ctx = MockEditableImage::from_nothing_context();
    ctx.expect()
        .with(mockall::predicate::eq(100u32), mockall::predicate::eq(100u32))
        .times(1)
        .returning(|_, _| {
            let mut mock = MockEditableImage::default();
            mock.expect_set_background_from_file()
                .with(mockall::predicate::eq("bg_multi1.png"))
                .times(1)
                .returning(|_| ());
            mock.expect_save()
                .with(mockall::predicate::str::starts_with("/out/test"))
                .times(1)
                .returning(|_| ());
            mock
        });
    ctx.expect()
        .with(mockall::predicate::eq(200u32), mockall::predicate::eq(200u32))
        .times(1)
        .returning(|_, _| {
            let mut mock = MockEditableImage::default();
            mock.expect_set_background_from_file()
                .with(mockall::predicate::eq("bg_multi2.png"))
                .times(1)
                .returning(|_| ());
            mock.expect_save()
                .with(mockall::predicate::str::starts_with("/out/test"))
                .times(1)
                .returning(|_| ());
            mock
        });

    let result = generator.generate(vec![recipe1, recipe2], "/out/test".to_string());
    assert!(result.is_ok());
}
