use mockall::automock;
use crate::infrastructure::filesystem::FileSystem;
use crate::models::image_recipe::{ImageRecipe, PrintableElementRecipe};
use rand::Rng;

#[automock]
pub trait ImageRecipeGenerator {
    fn generate(
        &self,
        count: u32,
        max_object_count_per_image: u32,
        max_distraction_count_per_image: u32,
    ) -> Result<Vec<ImageRecipe>, String>;
}

// ###################################### IMPL #####################################################

pub struct ImageRecipeGeneratorImpl<'a, FS: FileSystem> {
    filesystem: &'a FS,
    background_dir: String,
    object_dir: String,
    distraction_dir: Option<String>,
    width: u32,
    height: u32,
}

impl<'a, FS: FileSystem> ImageRecipeGeneratorImpl<'a, FS> {
    pub fn new(
        filesystem: &'a FS,
        background_dir: String,
        object_dir: String,
        distraction_dir: Option<String>,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            filesystem,
            background_dir,
            object_dir,
            distraction_dir,
            width,
            height,
        }
    }

    fn pick_background(&self) -> Result<String, String> {
        let backgrounds = self.filesystem.list_files(&self.background_dir)?;
        if backgrounds.is_empty() {
            return Err(format!(
                "No background images found in {}",
                self.background_dir
            ));
        }
        let index = Self::random(0, backgrounds.len() as u32 - 1) as usize;
        Ok(backgrounds[index].clone())
    }

    fn pick_object(
        &self,
    ) -> Result<PrintableElementRecipe, String> {

        let class_dir = self.filesystem.list_subdirectories(&self.object_dir)?;
        if class_dir.is_empty() {
            // NOTE: Class directory not found, pick directly from object directory and assume there are only one class
            Ok(self.pick_object_by_class(None)?)
        }else {
            let class = Self::random(0, class_dir.len() as u32 - 1);
            if self.filesystem.is_dir(&format!("{}/{}", self.object_dir, class)) {
                Ok(self.pick_object_by_class(Some(class))?)
            }else {
                Err(format!("Class directory {} not found", class))
            }
        }
    }

    fn pick_object_by_class(
        &self,
        class: Option<u32>,
    ) -> Result<PrintableElementRecipe, String> {
        let class_dir_path;
        if class.is_none() {
            class_dir_path = self.object_dir.clone();
        }else {
            class_dir_path = format!("{}/{}", self.object_dir, class.unwrap());
        }
        let objects = self.filesystem.list_files(&class_dir_path)?;
        if objects.is_empty() {
            return Err(format!("No object images found for class {} in {}", class.unwrap_or(0), self.object_dir));
        }
        let index = Self::random(0, objects.len() as u32 - 1) as usize;
        let mut object_recipe = self.build_element(objects[index].clone())
            .map_err(|err| format!("Failed to build object recipe: {}", err))?;
        object_recipe.class = class.unwrap_or(0);
        Ok(object_recipe)
    }

    fn pick_distraction(
        &self,
    ) -> Result<PrintableElementRecipe, String> {
        let dir = self
            .distraction_dir
            .as_ref()
            .ok_or_else(|| "No distraction directory configured".to_string())?;
        let distractions = self.filesystem.list_files(dir)?;
        if distractions.is_empty() {
            return Err(format!("No distraction images found in {}", dir));
        }

        let index = Self::random(0, distractions.len() as u32 - 1) as usize;
        self.build_element(distractions[index].clone())
            .map_err(|err| format!("Failed to build distraction recipe: {}", err))
    }

    fn build_element(&self, path: String) -> Result<PrintableElementRecipe, String> {
        let mut element = PrintableElementRecipe::default();

        let (element_width, element_height) = self.filesystem.get_image_size(&path)
            .map_err(|err| format!("Failed to get image size for {}: {}", path, err))?;

        element.path = path;
        element.size =  Self::random_f32(0.1, 0.7);
        element.angle = Self::random_f32(0.0, 360.0);

        let width_ratio = element_height as f32 / element_width as f32;
        let height_ratio = element_width as f32 / element_height as f32;

        let element_final_width = (self.width as f32 * element.size * width_ratio) as u32;
        let element_final_height = (self.height as f32 * element.size * height_ratio) as u32;

        let available_width = if self.width > element_final_width {
            self.width - element_final_width
        } else {
            1
        };
        let available_height = if self.height > element_final_height {
            self.height - element_final_height
        } else {
            1
        };

        element.x = Self::random(0, available_width);
        element.y = Self::random(0, available_height);

        Ok(element)
    }

    fn random(min: u32, max: u32) -> u32 {
        rand::thread_rng().gen_range(min..=max)
    }

    fn random_f32(min: f32, max: f32) -> f32 {
        rand::thread_rng().gen_range(min..max)
    }
}

impl<'a, FS: FileSystem> ImageRecipeGenerator for ImageRecipeGeneratorImpl<'a, FS>{
    fn generate(
        &self,
        count: u32,
        max_object_count_per_image: u32,
        max_distraction_count_per_image: u32,
    ) -> Result<Vec<ImageRecipe>, String> {
        let mut recipes: Vec<ImageRecipe> = Vec::new();

        for i in 0..count {
            let mut image = ImageRecipe::new();

            image.width = self.width;
            image.height = self.height;
            image.background_path = self.pick_background()?;

            let object_count = ImageRecipeGeneratorImpl::<FS>::random(1, max_object_count_per_image);
            for _ in 0..object_count {
                let object = self.pick_object()?;
                image.object.push(object);
            }

            if self.distraction_dir.is_some() {
                let mut distractions: Vec<PrintableElementRecipe> = Vec::new();
                let distraction_count =
                    ImageRecipeGeneratorImpl::<FS>::random(1, max_distraction_count_per_image);
                for _ in 0..distraction_count {
                    distractions.push(self.pick_distraction()?);
                }
                image.distraction = Some(distractions);
            }

            let id = Self::random(0, 1000000);
            image.name = format!("{}_{}", id, i);
            recipes.push(image);
        }

        Ok(recipes)
    }
}
