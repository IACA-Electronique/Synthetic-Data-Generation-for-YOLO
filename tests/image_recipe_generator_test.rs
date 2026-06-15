use mockall::predicate::eq;
use synthetic_data_generator_for_yolo::infrastructure::filesystem::MockFileSystem;
use synthetic_data_generator_for_yolo::services::image_recipe_generator::{
    ImageRecipeGenerator, ImageRecipeGeneratorImpl,
};

const OBJECT_COUNT_PER_IMAGE: u32 = 3;

fn build_generator(fs: &MockFileSystem) -> ImageRecipeGeneratorImpl<MockFileSystem> {
    ImageRecipeGeneratorImpl::new(
        fs,
        String::from("backgrounds"),
        String::from("objects"),
        Some("distractions".to_string()),
        640,
        480
    )
}

/// Builds a `MockFileSystem` that returns canned file lists for the
/// background, object and distraction directories.
/// Objects are organised in a single class sub-directory ("objects/0").
fn mock_filesystem() -> MockFileSystem {
    let mut filesystem = MockFileSystem::new();

    filesystem
        .expect_list_files()
        .with(eq("backgrounds"))
        .returning(|_| Ok(vec!["backgrounds/bg.png".to_string()]));

    filesystem
        .expect_list_subdirectories()
        .with(eq("objects"))
        .returning(|_| Ok(vec!["objects/0".to_string()]));

    filesystem
        .expect_list_files()
        .with(eq("objects/0"))
        .returning(|_| {
            Ok(vec![
                "objects/0/cat.png".to_string(),
                "objects/0/dog.png".to_string(),
            ])
        });

    filesystem
        .expect_is_dir()
        .with(eq("objects/0"))
        .returning(|_| true);

    filesystem
        .expect_list_files()
        .with(eq("distractions"))
        .returning(|_| Ok(vec!["distractions/noise.png".to_string()]));

    filesystem
}

#[test]
fn generate_returns_requested_number_of_recipes() {
    let filesystem = mock_filesystem();
    let generator = build_generator(&filesystem);

    let recipes = generator
        .generate(4)
        .expect("generation should succeed");

    assert_eq!(recipes.len(), 4);
}

#[test]
fn generate_with_zero_count_returns_empty_vec() {
    let mut filesystem = mock_filesystem();
    // No directory should be queried when nothing is generated.
    filesystem.expect_list_files().never();

    let generator = build_generator(&filesystem);

    let recipes = generator
        .generate(0)
        .expect("generation should succeed");

    assert!(recipes.is_empty());
}

#[test]
fn generate_sets_image_dimensions() {
    let filesystem = mock_filesystem();
    let generator = build_generator(&filesystem);

    let recipes = generator
        .generate(2)
        .expect("generation should succeed");

    for recipe in &recipes {
        assert_eq!(recipe.width, 640);
        assert_eq!(recipe.height, 480);
    }
}

#[test]
fn generate_uses_background_returned_by_filesystem() {
    let filesystem = mock_filesystem();
    let generator = build_generator(&filesystem);

    let recipes = generator
        .generate(1)
        .expect("generation should succeed");

    assert_eq!(recipes[0].background_path, "backgrounds/bg.png");
}

#[test]
fn generate_queries_background_directory() {
    let mut filesystem = MockFileSystem::new();

    // The background directory must be queried exactly once per image.
    filesystem
        .expect_list_files()
        .with(eq("backgrounds"))
        .times(1)
        .returning(|_| Ok(vec!["backgrounds/bg.png".to_string()]));
    filesystem
        .expect_list_subdirectories()
        .with(eq("objects"))
        .returning(|_| Ok(vec!["objects/0".to_string()]));
    filesystem
        .expect_list_files()
        .with(eq("objects/0"))
        .returning(|_| Ok(vec!["objects/0/cat.png".to_string()]));
    filesystem
        .expect_is_dir()
        .with(eq("objects/0"))
        .returning(|_| true);
    filesystem
        .expect_list_files()
        .with(eq("distractions"))
        .returning(|_| Ok(vec!["distractions/noise.png".to_string()]));

    let generator = build_generator(&mut filesystem);

    generator
        .generate(1)
        .expect("generation should succeed");
}


#[test]
fn generate_adds_objects_within_limit() {
    let filesystem = mock_filesystem();
    let generator = build_generator(&filesystem);

    let recipes = generator
        .generate(3)
        .expect("generation should succeed");

    for recipe in &recipes {
        assert!(!recipe.object.is_empty());
        assert!(recipe.object.len() as u32 <= OBJECT_COUNT_PER_IMAGE);
    }
}

#[test]
fn generate_adds_distractions_when_distraction_dir_is_set() {
    let filesystem = mock_filesystem();
    let generator = build_generator(&filesystem);

    let recipes = generator
        .generate( 2)
        .expect("generation should succeed");

    for recipe in &recipes {
        let distractions = recipe
            .distraction
            .as_ref()
            .expect("distractions should be set when distraction_dir is provided");
        assert!(!distractions.is_empty());
    }
}

#[test]
fn generate_without_distraction_dir_leaves_distraction_none() {
    let mut filesystem = MockFileSystem::new();

    filesystem
        .expect_list_files()
        .with(eq("backgrounds"))
        .returning(|_| Ok(vec!["backgrounds/bg.png".to_string()]));
    filesystem
        .expect_list_subdirectories()
        .with(eq("objects"))
        .returning(|_| Ok(vec!["objects/0".to_string()]));
    filesystem
        .expect_list_files()
        .with(eq("objects/0"))
        .returning(|_| Ok(vec!["objects/0/cat.png".to_string()]));
    filesystem
        .expect_is_dir()
        .with(eq("objects/0"))
        .returning(|_| true);
    // The distraction directory must never be queried when it is not configured.
    filesystem
        .expect_list_files()
        .with(eq("distractions"))
        .never();

    let generator = ImageRecipeGeneratorImpl::new(
        &filesystem,
        String::from("backgrounds"),
        String::from("objects"),
        None,
        640,
        480,
    );

    let recipes = generator
        .generate(1)
        .expect("generation should succeed");

    assert!(recipes[0].distraction.is_none());
}

#[test]
fn generate_propagates_filesystem_error() {
    let mut filesystem = MockFileSystem::new();
    filesystem
        .expect_list_files()
        .with(eq("backgrounds"))
        .returning(|_| Err("boom".to_string()));

    let generator = build_generator(&filesystem);
    let result = generator.generate(1);

    assert_eq!(result, Err("boom".to_string()));
}

#[test]
fn generate_picks_object_directly_when_no_sub_dir_class_exists() {
    // When list_subdirectories returns an empty vec, the generator should fall
    // back to picking files directly from the object directory (single-class mode).
    let mut filesystem = MockFileSystem::new();

    filesystem
        .expect_list_files()
        .with(eq("backgrounds"))
        .returning(|_| Ok(vec!["backgrounds/bg.png".to_string()]));

    filesystem
        .expect_list_subdirectories()
        .with(eq("objects"))
        .returning(|_| Ok(vec![])); // no sub-directories

    filesystem
        .expect_list_files()
        .with(eq("objects"))
        .returning(|_| Ok(vec!["objects/cat.png".to_string()]));

    filesystem
        .expect_list_files()
        .with(eq("distractions"))
        .returning(|_| Ok(vec!["distractions/noise.png".to_string()]));

    let generator = build_generator(&filesystem);

    let recipes = generator
        .generate(1)
        .expect("generation should succeed");

    assert!(!recipes[0].object.is_empty());
    // In single-class fallback the class id defaults to 0.
    for obj in &recipes[0].object {
        assert_eq!(obj.class, 0);
    }
}

#[test]
fn generate_error_when_sub_dir_class_does_not_exist() {
    // When list_subdirectories returns sub-dirs but is_dir returns false for the
    // chosen class path, the generator should propagate an error.
    let mut filesystem = MockFileSystem::new();

    filesystem
        .expect_list_files()
        .with(eq("backgrounds"))
        .returning(|_| Ok(vec!["backgrounds/bg.png".to_string()]));

    filesystem
        .expect_list_subdirectories()
        .with(eq("objects"))
        .returning(|_| Ok(vec!["objects/0".to_string()]));

    // The class directory does not actually exist.
    filesystem
        .expect_is_dir()
        .returning(|_| false);

    let generator = build_generator(&filesystem);

    let result = generator.generate(1);

    assert!(result.is_err());
}