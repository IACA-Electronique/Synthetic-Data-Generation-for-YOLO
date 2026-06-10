use crate::models::dataset_config::DatasetConfig;
use crate::models::image_recipe::ImageRecipe;
use crate::services::image_generator::ImageGenerator;
use crate::services::image_recipe_generator::ImageRecipeGenerator;

pub trait DataGeneratorOrchestrator {
    fn generate_images(&self,  count: u32, train_ratio: usize, val_ratio: usize, test_ratio: usize) -> Result<(), String>;
}

pub struct MultiThreadDataGeneratorOrchestrator<'a, R: ImageRecipeGenerator, I: ImageGenerator, C: DatasetConfig> {
    image_recipe_generator: &'a R,
    image_generator: &'a I,
    dataset_config: &'a C,
}

impl<'a, R: ImageRecipeGenerator, I: ImageGenerator, C: DatasetConfig> MultiThreadDataGeneratorOrchestrator<'a, R, I, C> {
    pub fn new(image_recipe_generator: &'a R, image_generator: &'a I, dataset_config: &'a C) -> Self {
        Self { image_recipe_generator, image_generator, dataset_config }
    }
}

impl<'a, R: ImageRecipeGenerator, I: ImageGenerator, C: DatasetConfig> DataGeneratorOrchestrator for MultiThreadDataGeneratorOrchestrator<'a, R, I, C> {
    fn generate_images(&self, count: u32, train_ratio: usize, val_ratio: usize, test_ratio: usize) -> Result<(), String> {
        // let recipes: Vec<ImageRecipe> = self.image_recipe_generator.generate(count)
        //     .map_err(|e| format!("Failed to generate image recipes: {}", e))?;

        // TODO:
        //  - Change in image generator filesystem injection to pass by constructor.
        //  - Split recipes in sub vec respectively to train, val and test ratio.
        //  - Create a message queue to send the recipes to a pool of image generator workers (pool size configurable in contructor).
        //  - Each worker take a recipe, generate the image and generate the label file (label file generator to be implemented).
        //  - generate_images wait that all workers are done.
        //  - handle cancel (add observable cancel flag in constructor ?)
        //  - Write unit tests.

        todo!()
    }
}