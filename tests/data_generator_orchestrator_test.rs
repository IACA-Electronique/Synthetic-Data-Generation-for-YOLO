use std::sync::LazyLock;
use synthetic_data_generator_for_yolo::infrastructure::filesystem::MockFileSystem;
use synthetic_data_generator_for_yolo::models::dataset_config::MockDatasetConfig;
use synthetic_data_generator_for_yolo::models::image_recipe::ImageRecipe;
use synthetic_data_generator_for_yolo::services::data_generator_orchestrator::MultiThreadDataGeneratorOrchestrator;
use synthetic_data_generator_for_yolo::services::image_generator::MockImageGenerator;
use synthetic_data_generator_for_yolo::services::image_recipe_generator::MockImageRecipeGenerator;

static FS: LazyLock<MockFileSystem> = LazyLock::new(MockFileSystem::new);
static IG: LazyLock<MockImageGenerator> = LazyLock::new(MockImageGenerator::new);
static IGG: LazyLock<MockImageRecipeGenerator> = LazyLock::new(MockImageRecipeGenerator::new);
static DC: LazyLock<MockDatasetConfig> = LazyLock::new(MockDatasetConfig::new);


fn build_orch(
) -> MultiThreadDataGeneratorOrchestrator<
    'static,
    MockImageRecipeGenerator,
    MockImageGenerator,
    MockDatasetConfig,
    MockFileSystem,
> {
    MultiThreadDataGeneratorOrchestrator::new(&IGG, &IG, &DC, &FS)
}

// ------------------------------------------------------------------------------------------------

fn make_recipes(n: usize) -> Vec<ImageRecipe> {
    (0..n).map(|_| ImageRecipe::new()).collect()
}

// --- success cases ---

#[test]
fn split_recipes_produces_correctly_sized_subsets_for_standard_ratios() {
    let recipes = make_recipes(100);

    let orch = build_orch();

    let (train, val, test) = orch.split_recipes(recipes, 80, 10).unwrap();

    assert_eq!(train.len(), 80);
    assert_eq!(val.len(), 10);
    assert_eq!(test.len(), 10);
}

#[test]
fn split_recipes_produces_correctly_sized_subsets_for_small_dataset() {
    // 10 recipes: 60% train=6, 20% val=2 from remaining 4, test=2
    let recipes = make_recipes(10);

    let (train, val, test) = build_orch().split_recipes(recipes, 60, 20).unwrap();

    assert_eq!(train.len(), 6);
    assert_eq!(val.len(), 2);
    assert_eq!(test.len(), 2);
}

// --- failure cases ---

#[test]
fn split_recipes_returns_error_when_train_subset_is_empty() {
    let recipes = make_recipes(100);

    let result = build_orch().split_recipes(recipes, 0, 10);

    assert!(result.is_err());
}

#[test]
fn split_recipes_returns_error_when_val_subset_is_empty() {
    let recipes = make_recipes(100);

    let result = build_orch().split_recipes(recipes, 80, 0);

    assert!(result.is_err());
}

#[test]
fn split_recipes_returns_error_when_test_subset_is_empty() {
    // 10 recipes: 90% train=9, 10% val=floor(0.3)=0 → val empty
    let recipes = make_recipes(10);

    let result = build_orch().split_recipes(recipes, 90, 10);

    assert!(result.is_err());
}

#[test]
fn split_recipes_returns_error_when_dataset_is_too_small_for_given_ratios() {
    // 3 recipes: 80% train=2, 10% val=floor(0.3)=0 → val empty
    let recipes = make_recipes(3);

    let result = build_orch().split_recipes(recipes, 80, 10);

    assert!(result.is_err());
}
