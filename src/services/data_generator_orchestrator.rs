use crate::models::dataset_config::DatasetConfig;
use crate::services::image_generator::ImageGenerator;
use crate::services::image_recipe_generator::ImageRecipeGenerator;
use mockall::automock;
use crate::infrastructure::filesystem::FileSystem;
use crate::models::image_recipe::ImageRecipe;

#[automock]
pub trait DataGeneratorOrchestrator {
    fn generate_images(&self,  count: u32, train_ratio: usize, val_ratio: usize, test_ratio: usize) -> Result<(), String>;
}

pub struct MultiThreadDataGeneratorOrchestrator<'a, R: ImageRecipeGenerator, I: ImageGenerator, C: DatasetConfig, FS: FileSystem> {
    image_recipe_generator: &'a R,
    image_generator: &'a I,
    dataset_config: &'a C,
    filesystem: &'a FS,
}

impl<'a, R: ImageRecipeGenerator, I: ImageGenerator, C: DatasetConfig, FS: FileSystem> MultiThreadDataGeneratorOrchestrator<'a, R, I, C, FS> {
    pub fn new(image_recipe_generator: &'a R, image_generator: &'a I, dataset_config: &'a C, filesystem: &'a FS) -> Self {
        Self { image_recipe_generator, image_generator, dataset_config, filesystem }
    }

    pub fn split_recipes(&self, recipes: Vec<ImageRecipe>, train_ratio: usize, val_ratio: usize) -> Result<(Vec<ImageRecipe>, Vec<ImageRecipe>, Vec<ImageRecipe>), String> {
        let train_size = (recipes.len() as f32 * train_ratio as f32 / 100.0) as usize;
        let val_size = (recipes.len() as f32 * val_ratio as f32 / 100.0) as usize;

        let (train_recipes, remaining_recipes) = recipes.split_at(train_size);
        let (val_recipes, test_recipes) = remaining_recipes.split_at(val_size);

        if train_recipes.is_empty() || val_recipes.is_empty() || test_recipes.is_empty() {
            Err("Failed to split recipes, some of subsets are empty. Maybe dataset is too small ?".to_string())
        }else {
            Ok((train_recipes.to_vec(), val_recipes.to_vec(), test_recipes.to_vec()))
        }
    }
}

impl<'a, R: ImageRecipeGenerator, I: ImageGenerator, C: DatasetConfig, FS: FileSystem> DataGeneratorOrchestrator for MultiThreadDataGeneratorOrchestrator<'a, R, I, C, FS> {
    fn generate_images(&self, count: u32, train_ratio: usize, val_ratio: usize, test_ratio: usize) -> Result<(), String> {
        let recipes: Vec<ImageRecipe> = self.image_recipe_generator.generate(count)
            .map_err(|e| format!("Failed to generate image recipes: {}", e))?;

        let (train_recipes, val_recipes, test_recipes) =
            self.split_recipes(recipes, train_ratio, val_ratio)?;


        // TODO:
        //  - Split recipes in sub vec respectively to train, val and test ratio.
        //  - Create a message queue to send the recipes to a pool of image generator workers (pool size configurable in contructor).
        //  - Each worker take a recipe, generate the image and generate the label file (label file generator to be implemented).
        //  - generate_images wait that all workers are done.
        //  - handle cancel (add observable cancel flag in constructor ?)
        //  - Write unit tests.

        todo!()
    }
}