use crate::infrastructure::filesystem::FileSystem;
use crate::models::dataset_config::DatasetConfig;
use crate::models::image_recipe::ImageRecipe;
use crate::services::image_generator::ImageGenerator;
use crate::services::image_recipe_generator::ImageRecipeGenerator;
use crate::services::label_generator::LabelGenerator;
use async_trait::async_trait;
use mockall::automock;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

#[automock]
#[async_trait]
pub trait DataGeneratorOrchestrator {
    async fn generate_images<'cb>(
        &self,
        count: u32,
        max_object_count_per_image: u32,
        max_distraction_count_per_image: u32,
        train_ratio: usize,
        val_ratio: usize,
        test_ratio: usize,
        thread_count: usize,
        on_progress: Option<GenerateImagesProgressCallback<'cb>>) -> Result<(), String>;
}

#[derive(Debug, Clone)]
pub enum GenerateImagesProgress {
    Started {
        total: u32,
    },
    RecipesGenerated {
        total: u32,
    },
    Generating {
        count: u32,
        total: u32,
    },
    Completed {
        total: u32,
    },
}

pub type GenerateImagesProgressCallback<'a> =
&'a (dyn Fn(GenerateImagesProgress) + Send + Sync);

pub struct MultiThreadDataGeneratorOrchestrator<
    'a,
    R: ImageRecipeGenerator + Sync,
    I: ImageGenerator + Sync,
    L: LabelGenerator + Sync,
    C: DatasetConfig + Sync,
    FS: FileSystem + Sync,
> {
    image_recipe_generator: &'a R,
    image_generator: &'a I,
    label_generator: &'a L,
    dataset_config: &'a C,
    filesystem: PhantomData<FS>
}

impl<
    'a,
    R: ImageRecipeGenerator + Sync,
    I: ImageGenerator + Sync,
    L: LabelGenerator + Sync,
    C: DatasetConfig + Sync,
    FS: FileSystem + Sync,
> MultiThreadDataGeneratorOrchestrator<'a, R, I, L, C, FS> {
    pub fn new(image_recipe_generator: &'a R, image_generator: &'a I, label_generator: &'a L, dataset_config: &'a C) -> Self {
        Self { image_recipe_generator, image_generator, label_generator, dataset_config, filesystem: PhantomData }
    }

    fn log_process_started(on_progress: Option<GenerateImagesProgressCallback>, count: u32) {
        if let Some(callback) = on_progress {
            callback(GenerateImagesProgress::Started {
                total: count,
            });
        }
    }

    fn log_process_done(on_progress: Option<GenerateImagesProgressCallback>, pool_total_count: u32) {
        if let Some(callback) = on_progress {
            callback(GenerateImagesProgress::Completed {
                total: pool_total_count,
            });
        }
    }

    fn log_recipes_generation_done(on_progress: Option<GenerateImagesProgressCallback>, total_recipes: u32) {
        if let Some(callback) = on_progress {
            callback(GenerateImagesProgress::RecipesGenerated {
                total: total_recipes,
            });
        }
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

    pub fn get_output_dir_path_from_datatype(&self, datatype: DataType) -> (String, String) {
        let output_dir_paths: (String, String) ;
        if datatype == DataType::TRAIN {
            output_dir_paths = (self.dataset_config.get_images_train_dir_path().to_string(),
            self.dataset_config.get_labels_train_dir_path().to_string());
        } else if datatype == DataType::VAL {
            output_dir_paths = (self.dataset_config.get_images_val_dir_path().to_string(),
            self.dataset_config.get_labels_val_dir_path().to_string());
        } else {
            output_dir_paths = (self.dataset_config.get_images_test_dir_path().to_string(),
            self.dataset_config.get_labels_test_dir_path().to_string());
        }
        output_dir_paths
    }

    fn split_pool_for_threads(pool: Vec<(ImageRecipe, DataType)>, thread_count: usize) -> Vec<Vec<(ImageRecipe, DataType)>> {
        let chunk_size = (pool.len() + thread_count - 1) / thread_count; // ceil division

        let subpools: Vec<Vec<(ImageRecipe, DataType)>> = pool
            .chunks(chunk_size)
            .map(|c| c.to_vec())
            .collect();
        subpools
    }

    fn build_pool_of_recipes(&self, train_ratio: usize, val_ratio: usize, recipes: Vec<ImageRecipe>) -> Result<Vec<(ImageRecipe, DataType)>, String> {
        let (train_recipes, val_recipes, test_recipes) =
            self.split_recipes(recipes, train_ratio, val_ratio)?;

        let mut pool: Vec<(ImageRecipe, DataType)> = Vec::new();

        for recipe in train_recipes {
            pool.push((recipe, DataType::TRAIN));
        }

        for recipe in val_recipes {
            pool.push((recipe, DataType::VAL));
        }

        for recipe in test_recipes {
            pool.push((recipe, DataType::TEST));
        }
        Ok(pool)
    }

    fn run_process_in_threads(&self, on_progress: Option<GenerateImagesProgressCallback>, subpools: Vec<Vec<(ImageRecipe, DataType)>>, pool_total_count: u32) -> Result<(), String> {
        let this = &*self;
        let counter: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));

        std::thread::scope(|s| -> Result<(), String> {
            let mut handles = Vec::with_capacity(subpools.len());
            for subpool in subpools {
                let counter = Arc::clone(&counter);
                let handle = s.spawn(move || -> Result<(), String> {
                    for (recipe, datatype) in subpool {
                        let (output_dir_path, label_dir_path) =
                            this.get_output_dir_path_from_datatype(datatype);

                        this.image_generator
                            .generate_one(recipe.clone(), output_dir_path)?;
                        this.label_generator
                            .generate_one(recipe, label_dir_path)?;

                        let mut i = counter.lock().unwrap();
                        *i += 1;
                        let current = i.clone();
                        if let Some(callback) = on_progress {
                            callback(GenerateImagesProgress::Generating {
                                count: current,
                                total: pool_total_count,
                            });
                        }
                    }
                    Ok(())
                });
                handles.push(handle);
            }

            // Join all threads and propagate the first error if any.
            for h in handles {
                h.join().map_err(|_| "Worker thread panicked".to_string())??;
            }

            Ok(())
        })?;
        Ok(())
    }
}

#[async_trait]
impl<
    'a,
    R: ImageRecipeGenerator + Sync,
    I: ImageGenerator + Sync,
    L: LabelGenerator + Sync,
    C: DatasetConfig + Sync,
    FS: FileSystem + Sync,
> DataGeneratorOrchestrator for MultiThreadDataGeneratorOrchestrator<'a, R, I, L, C, FS> {
    async fn generate_images<'cb>(
        &self,
        count: u32,
        max_object_count_per_image: u32,
        max_distraction_count_per_image: u32,
        train_ratio: usize,
        val_ratio: usize,
        _test_ratio: usize,
        thread_count: usize,
        on_progress: Option<GenerateImagesProgressCallback<'cb>>) -> Result<(), String> {

        if count == 0 {
            return Err("Count must be greater than 0".to_string());
        }

        if thread_count == 0 {
            return Err("Thread count must be greater than 0".to_string());
        }

        if thread_count as u32 > count {
            return Err("Thread count must be less than or equal to the total number of images".to_string());
        }

        Self::log_process_started(on_progress, count);

        let recipes: Vec<ImageRecipe> = self.image_recipe_generator.generate(count, max_object_count_per_image, max_distraction_count_per_image)
            .map_err(|e| format!("Failed to generate image recipes: {}", e))?;

        Self::log_recipes_generation_done(on_progress, recipes.len() as u32);

        let pool = self.build_pool_of_recipes(train_ratio, val_ratio, recipes)?;

        let pool_total_count = pool.len() as u32;

        let subpools = Self::split_pool_for_threads(pool, thread_count);

        self.run_process_in_threads(on_progress, subpools, pool_total_count)?;

        Self::log_process_done(on_progress, pool_total_count);

        Ok(())
    }
}


#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum DataType{
    TRAIN,
    VAL,
    TEST,
}