use crate::infrastructure::filesystem::FileSystem;
use crate::models::image_recipe::{ImageRecipe, PrintableElementRecipe};
use rand::Rng;

pub trait ImageRecipeGenerator {
    fn create(
        background_dir: &str,
        object_dir: &str,
        distraction_dir: Option<String>,
        width: u32,
        height: u32,
        output_dir: String,
    ) -> Self;
    fn generate<FS: FileSystem>(
        &self,
        filesystem: &FS,
        count: u32,
    ) -> Result<Vec<ImageRecipe>, String>;
}

// ###################################### IMPL #####################################################

const OBJECT_COUNT_PER_IMAGE: u32 = 3;
const DISTRACTION_COUNT_PER_IMAGE: u32 = 2;

pub struct ImageRecipeGeneratorImpl {
    background_dir: String,
    object_dir: String,
    distraction_dir: Option<String>,
    width: u32,
    height: u32,
    output_dir: String,
}

impl ImageRecipeGeneratorImpl {
    pub fn new(
        background_dir: String,
        object_dir: String,
        distraction_dir: Option<String>,
        width: u32,
        height: u32,
        output_dir: String,
    ) -> Self {
        Self {
            background_dir,
            object_dir,
            distraction_dir,
            width,
            height,
            output_dir,
        }
    }

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

        let class_dir = filesystem.list_subdirectories(&self.object_dir)?;
        if class_dir.is_empty() {
            // NOTE: Class directory not found, pick directly from object directory and assume there are only one class
            Ok(self.pick_object_by_class(filesystem, None)?)
        }else {
            let class = Self::random(0, class_dir.len() as u32 - 1);
            if filesystem.is_dir(&format!("{}/{}", self.object_dir, class)) {
                Ok(self.pick_object_by_class(filesystem, Some(class))?)
            }else {
                Err(format!("Class directory {} not found", class))
            }
        }
    }

    fn pick_object_by_class<FS: FileSystem>(
        &self,
        filesystem: &FS,
        class: Option<u32>,
    ) -> Result<PrintableElementRecipe, String> {
        let class_dir_path;
        if class.is_none() {
            class_dir_path = self.object_dir.clone();
        }else {
            class_dir_path = format!("{}/{}", self.object_dir, class.unwrap());
        }
        let objects = filesystem.list_files(&class_dir_path)?;
        if objects.is_empty() {
            return Err(format!("No object images found for class {} in {}", class.unwrap_or(0), self.object_dir));
        }
        let index = Self::random(0, objects.len() as u32 - 1) as usize;
        let mut object_recipe = self.build_element(objects[index].clone());
        object_recipe.id = class.unwrap_or(0);
        Ok(object_recipe)
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
        let mut element = PrintableElementRecipe::default();

        element.path = path;
        element.size =  Self::random_f32(0.1, 1.0);
        element.angle = Self::random_f32(0.0, 360.0);
        element.x = Self::random(0, self.width);
        element.y = Self::random(0, self.height);

        element
    }

    fn random(min: u32, max: u32) -> u32 {
        rand::thread_rng().gen_range(min..=max)
    }

    fn random_f32(min: f32, max: f32) -> f32 {
        rand::thread_rng().gen_range(min..max)
    }
}

impl ImageRecipeGenerator for ImageRecipeGeneratorImpl {
    fn create(
        background_dir: &str,
        object_dir: &str,
        distraction_dir: Option<String>,
        width: u32,
        height: u32,
        output_dir: String,
    ) -> Self {
        ImageRecipeGeneratorImpl {
            background_dir: background_dir.to_string(),
            object_dir: object_dir.to_string(),
            distraction_dir,
            width,
            height,
            output_dir,
        }
    }

    fn generate<FS: FileSystem>(
        &self,
        filesystem: &FS,
        count: u32,
    ) -> Result<Vec<ImageRecipe>, String> {
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
                let mut distractions: Vec<PrintableElementRecipe> = Vec::new();
                let distraction_count =
                    ImageRecipeGeneratorImpl::random(1, DISTRACTION_COUNT_PER_IMAGE);
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
