use rand::prelude::ThreadRng;
use rand::Rng;
use crate::infrastructure::filesystem::FileSystem;
use crate::models::image_recipe::{ImageRecipe, PrintableElementRecipe};

pub trait ImageRecipeGenerator {
    fn create(
        background_dir: &str,
        object_dir: &str,
        distraction_dir: Option<String>,
        width: u32,
        height: u32,
        output_dir: String,
    ) -> Self;
    fn generate<FS: FileSystem>(&self, count: u32) -> Result<Vec<ImageRecipe>, String>;
}

// ###################################### IMPL #####################################################

const OBJECT_COUNT_PER_IMAGE: u32 = 3;
const DISTRACTION_COUNT_PER_IMAGE: u32 = 2;

pub struct ImageRecipeGeneratorImpl {
    background_dir: String,
    object_dir: String,
    distraction_dir: Option<String>,
    width: u32, height: u32,
    output_dir: String,
}

impl  ImageRecipeGeneratorImpl {

    fn pick_background(&self) -> Result<String, String> {
        todo!()
    }

    fn pick_object(&self) -> Result<PrintableElementRecipe, String> {
        todo!()
    }

    fn pick_distraction(&self) -> Result<PrintableElementRecipe, String> {
        todo!()
    }

    fn random(min: u32, max: u32) -> u32 {
        rand::thread_rng().gen_range(min..=max)
    }
}

impl ImageRecipeGenerator for ImageRecipeGeneratorImpl {
    fn create(background_dir: &str, object_dir: &str, distraction_dir: Option<String>, width: u32, height: u32, output_dir: String) -> Self {
        ImageRecipeGeneratorImpl {
            background_dir: background_dir.to_string(),
            object_dir: object_dir.to_string(),
            distraction_dir,
            width, height, output_dir,
        }
    }

    fn generate<FS: FileSystem>(&self, count: u32) -> Result<Vec<ImageRecipe>, String> {
        let mut recipes: Vec<ImageRecipe> = Vec::new();

        for i in 0..count {
            let mut image = ImageRecipe::new();

            image.width = self.width;
            image.height = self.height;
            image.background_path = self.pick_background()?;

            let object_count = ImageRecipeGeneratorImpl::random(1, OBJECT_COUNT_PER_IMAGE);
            for _ in 0..object_count {
                let object = self.pick_object()?;
                image.object.push(object);
            }

            if self.distraction_dir.is_some() {
                let mut distractions : Vec<PrintableElementRecipe>= Vec::new();
                let distraction_count = ImageRecipeGeneratorImpl::random(1, DISTRACTION_COUNT_PER_IMAGE);
                for _ in 0..distraction_count {
                    distractions.push(self.pick_distraction()?);
                }
            }

            let id = Self::random(0, 1000000);
            image.output_path = format!("{}/{}_{}.png", self.output_dir, id, i);
            recipes.push(image);
        }

        Ok(recipes)
    }
}