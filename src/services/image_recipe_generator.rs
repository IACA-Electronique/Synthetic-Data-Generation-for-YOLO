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
    fn generate<FS: FileSystem>(&self, filesystem: &FS, count: u32) -> Result<Vec<ImageRecipe>, String>;
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

    fn pick_background<FS: FileSystem>(&self, filesystem: &FS) -> Result<String, String> {
        let backgrounds = filesystem.list_files(&self.background_dir)?;
        if backgrounds.is_empty() {
            return Err(format!(
                "No background images found in {}",
                self.background_dir
            ));
        }
        let index = Self::random(0, backgrounds.len() as u32 - 1) as usize;
        Ok(backgrounds[index].clone())
    }

    fn pick_object<FS: FileSystem>(
        &self,
        filesystem: &FS,
    ) -> Result<PrintableElementRecipe, String> {
        let objects = filesystem.list_files(&self.object_dir)?;
        if objects.is_empty() {
            return Err(format!("No object images found in {}", self.object_dir));
        }
        let index = Self::random(0, objects.len() as u32 - 1) as usize;
        Ok(self.build_element(objects[index].clone()))
    }

    fn pick_distraction<FS: FileSystem>(
        &self,
        filesystem: &FS,
    ) -> Result<PrintableElementRecipe, String> {
        let dir = self
            .distraction_dir
            .as_ref()
            .ok_or_else(|| "No distraction directory configured".to_string())?;
        let distractions = filesystem.list_files(dir)?;
        if distractions.is_empty() {
            return Err(format!("No distraction images found in {}", dir));
        }
        let index = Self::random(0, distractions.len() as u32 - 1) as usize;
        Ok(self.build_element(distractions[index].clone()))
    }

    fn build_element(&self, path: String) -> PrintableElementRecipe {
        let id = Self::random(0, 1_000_000);
        let size = Self::random_f32(0.1, 1.0);
        let angle = Self::random_f32(0.0, 360.0);
        let x = Self::random(0, self.width);
        let y = Self::random(0, self.height);
        PrintableElementRecipe::new(path, id, size, angle, x, y)
    }

    fn random(min: u32, max: u32) -> u32 {
        rand::thread_rng().gen_range(min..=max)
    }

    fn random_f32(min: f32, max: f32) -> f32 {
        rand::thread_rng().gen_range(min..max)
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

    fn generate<FS: FileSystem>(&self, filesystem: &FS, count: u32) -> Result<Vec<ImageRecipe>, String> {
        let mut recipes: Vec<ImageRecipe> = Vec::new();

        for i in 0..count {
            let mut image = ImageRecipe::new();

            image.width = self.width;
            image.height = self.height;
            image.background_path = self.pick_background(filesystem)?;

            let object_count = ImageRecipeGeneratorImpl::random(1, OBJECT_COUNT_PER_IMAGE);
            for _ in 0..object_count {
                let object = self.pick_object(filesystem)?;
                image.object.push(object);
            }

            if self.distraction_dir.is_some() {
                let mut distractions : Vec<PrintableElementRecipe>= Vec::new();
                let distraction_count = ImageRecipeGeneratorImpl::random(1, DISTRACTION_COUNT_PER_IMAGE);
                for _ in 0..distraction_count {
                    distractions.push(self.pick_distraction(filesystem)?);
                }
                image.distraction = Some(distractions);
            }

            let id = Self::random(0, 1000000);
            image.output_path = format!("{}/{}_{}.png", self.output_dir, id, i);
            recipes.push(image);
        }

        Ok(recipes)
    }
}