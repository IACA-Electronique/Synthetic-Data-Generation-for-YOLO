use synthetic_data_generator_for_yolo::infrastructure::filesystem::MockFileSystem;
use synthetic_data_generator_for_yolo::services::image_recipe_generator::{
    ImageRecipeGenerator, ImageRecipeGeneratorImpl,
};

const OBJECT_COUNT_PER_IMAGE: u32 = 3;

fn build_generator() -> ImageRecipeGeneratorImpl {
    ImageRecipeGeneratorImpl::create(
        "backgrounds",
        "objects",
        Some("distractions".to_string()),
        640,
        480,
        "output".to_string(),
    )
}

#[test]
fn generate_returns_requested_number_of_recipes() {
    let generator = build_generator();

    let recipes = generator
        .generate::<MockFileSystem>(4)
        .expect("generation should succeed");

    assert_eq!(recipes.len(), 4);
}

#[test]
fn generate_with_zero_count_returns_empty_vec() {
    let generator = build_generator();

    let recipes = generator
        .generate::<MockFileSystem>(0)
        .expect("generation should succeed");

    assert!(recipes.is_empty());
}

#[test]
fn generate_sets_image_dimensions() {
    let generator = build_generator();

    let recipes = generator
        .generate::<MockFileSystem>(2)
        .expect("generation should succeed");

    for recipe in &recipes {
        assert_eq!(recipe.width, 640);
        assert_eq!(recipe.height, 480);
    }
}

#[test]
fn generate_sets_a_background_path() {
    let generator = build_generator();

    let recipes = generator
        .generate::<MockFileSystem>(1)
        .expect("generation should succeed");

    assert!(!recipes[0].background_path.is_empty());
}

#[test]
fn generate_sets_output_path_in_output_dir() {
    let generator = build_generator();

    let recipes = generator
        .generate::<MockFileSystem>(1)
        .expect("generation should succeed");

    assert!(recipes[0].output_path.starts_with("output/"));
    assert!(recipes[0].output_path.ends_with(".png"));
}

#[test]
fn generate_adds_objects_within_limit() {
    let generator = build_generator();

    let recipes = generator
        .generate::<MockFileSystem>(3)
        .expect("generation should succeed");

    for recipe in &recipes {
        assert!(!recipe.object.is_empty());
        assert!(recipe.object.len() as u32 <= OBJECT_COUNT_PER_IMAGE);
    }
}

#[test]
fn generate_adds_distractions_when_distraction_dir_is_set() {
    let generator = build_generator();

    let recipes = generator
        .generate::<MockFileSystem>(2)
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
    let generator = ImageRecipeGeneratorImpl::create(
        "backgrounds",
        "objects",
        None,
        640,
        480,
        "output".to_string(),
    );

    let recipes = generator
        .generate::<MockFileSystem>(1)
        .expect("generation should succeed");

    assert!(recipes[0].distraction.is_none());
}
