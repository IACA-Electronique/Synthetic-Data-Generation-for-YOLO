use crate::infrastructure::filesystem::FileSystem;
use crate::models::dataset_config::DatasetConfig;
use crate::models::image_recipe::ImageRecipe;
use crate::services::image_generator::ImageGenerator;
use crate::services::image_recipe_generator::ImageRecipeGenerator;
use mockall::automock;
use std::marker::PhantomData;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use crate::services::label_generator::LabelGenerator;

#[automock]
#[async_trait]
pub trait DataGeneratorOrchestrator {
    async fn generate_images<'cb>(&self,  count: u32, train_ratio: usize, val_ratio: usize, test_ratio: usize, on_progress: Option<GenerateImagesProgressCallback<'cb>>) -> Result<(), String>;
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
    async fn generate_images<'cb>(&self, count: u32, train_ratio: usize, val_ratio: usize, _test_ratio: usize,on_progress: Option<GenerateImagesProgressCallback<'cb>>) -> Result<(), String> {
        if let Some(callback) = on_progress {
            callback(GenerateImagesProgress::Started {
                total: count,
            });
        }

        let recipes: Vec<ImageRecipe> = self.image_recipe_generator.generate(count)
            .map_err(|e| format!("Failed to generate image recipes: {}", e))?;

        if let Some(callback) = on_progress {
            callback(GenerateImagesProgress::RecipesGenerated {
                total: recipes.len() as u32,
            });
        }

        let (train_recipes, val_recipes, test_recipes) =
            self.split_recipes(recipes, train_ratio, val_ratio)?;

        let mut pool : Vec<(ImageRecipe, DataType)> = Vec::new();

        for recipe in train_recipes {
            pool.push((recipe, DataType::TRAIN));
        }

        for recipe in val_recipes {
            pool.push((recipe, DataType::VAL));
        }

        for recipe in test_recipes {
            pool.push((recipe, DataType::TEST));
        }

        let count = pool.len() as u32;
        let mut i = 0;

        let counter: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
        let thread_count = 8;
        let chunk_size = (pool.len() + thread_count - 1) / thread_count; // ceil division

        // Split pool into 4 (roughly) equal subpools.
        let subpools: Vec<Vec<(ImageRecipe, DataType)>> = pool
            .chunks(chunk_size)
            .map(|c| c.to_vec())
            .collect();

        // Borrow references for the threads.
        let this = &*self;

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
                                total: count,
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


        if let Some(callback) = on_progress {
            callback(GenerateImagesProgress::Completed {
                total: count,
            });
        }

        Ok(())
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum DataType{
    TRAIN,
    VAL,
    TEST,
}