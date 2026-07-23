use crate::infrastructure::filesystem::FileSystem;
use crate::models::image_recipe::{ImageRecipe, PrintableElementRecipe};
use mockall::automock;
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

    fn try_to_pick_object(
        &self,
        object_cache: &mut Vec<PrintableElementRecipe>,
    ) -> Result<Option<PrintableElementRecipe>, String> {

        let class_dir = self.filesystem.list_subdirectories(&self.object_dir)?;
        if class_dir.is_empty() {
            // NOTE: Class directory not found, pick directly from object directory and assume there are only one class
            let object_picked = self.pick_object_by_class(None, object_cache)?;
            Ok(object_picked)
        }else {
            let class = Self::random(0, class_dir.len() as u32 - 1);
            if self.filesystem.is_dir(&format!("{}/{}", self.object_dir, class)) {
                let object_picked = self.pick_object_by_class(Some(class), object_cache)?;
                Ok(object_picked)
            }else {
                Err(format!("Class directory {} not found", class))
            }
        }
    }

    fn pick_object_by_class(
        &self,
        class: Option<u32>,
        object_cache: &mut Vec<PrintableElementRecipe>,
    ) -> Result<Option<PrintableElementRecipe>, String> {
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
        let mut object_recipe_option = self.build_element(objects[index].clone(), Some(object_cache))
            .map_err(|err| format!("Failed to build object recipe: {}", err))?;
        if object_recipe_option.is_none() {
            Ok(None)
        }else {
            let object_recipe = object_recipe_option.as_mut().unwrap();
            object_recipe.class = class.unwrap_or(0);
            Ok(Some(object_recipe.clone()))
        }
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
        let distraction = self.build_element(distractions[index].clone(), None)
            .map_err(|err| format!("Failed to build distraction recipe: {}", err))?;
        if distraction.is_none() {
            Err("Failed to build distraction recipe".to_string())
        }else {
            Ok(distraction.unwrap())
        }
    }

    fn build_element(&self, path: String, cache: Option<&mut Vec<PrintableElementRecipe>>) -> Result<Option<PrintableElementRecipe>, String> {
        if let Some(cache) = cache {
             match self.build_element_with_cache_collision_check(path.clone(), cache) {
                Err(e) =>  {
                    eprintln!("Warning: {}", e);
                    Ok(None)
                }
                Ok(element) =>{
                    Ok(Some(element))
                }
            }
        }else {
            match self.generate_element(path) {
                Err(e) =>  {
                    Err(e)
                }
                Ok(element) =>{
                    Ok(Some(element))
                }
            }
        }
    }

    fn build_element_with_cache_collision_check(&self, path: String, cache: &mut Vec<PrintableElementRecipe>) -> Result<PrintableElementRecipe, String> {
        const MAX_PLACEMENT_ATTEMPTS: u32 = 50;
        let mut i = 0;
        let mut final_element: PrintableElementRecipe = self.generate_element(path.clone())?;
        let mut placed = false;

        while !placed && i < MAX_PLACEMENT_ATTEMPTS {
            if self.has_collision(&final_element, cache) {
                let current_element = self.generate_element(path.clone())?;
                final_element = current_element;
            }else {
                placed = true;
            }

            i += 1;
        }

        if !placed && i > 0 {
            Err(format!("Could not find collision-free position after {} attempts for {}", MAX_PLACEMENT_ATTEMPTS, path))
        }else {
            cache.push(final_element.clone());
            Ok(final_element)
        }
    }

    fn generate_element(&self, path: String) -> Result<PrintableElementRecipe, String> {
        let mut element = PrintableElementRecipe::default();

        let (element_width, element_height) = self.filesystem.get_image_size(&path)
            .map_err(|err| format!("Failed to get image size for {}: {}", path, err))?;

        element.path = path;
        let size =  Self::random_f32(0.1, 0.7);
        element.angle = Self::random_f32(0.0, 360.0);

        let width_ratio = element_width as f32 / element_height as f32;
        let height_ratio = element_height as f32 / element_width as f32;

        element.width = (self.width as f32 * size * width_ratio) as u32;
        element.height = (self.height as f32 * size * height_ratio) as u32;

        let available_width = if self.width > element.width {
            self.width - element.width
        } else {
            1
        };
        let available_height = if self.height > element.height {
            self.height - element.height
        } else {
            1
        };

        element.x = Self::random(0, available_width);
        element.y = Self::random(0, available_height);

        Ok(element)
    }

    fn has_collision(
        &self,
        element: &PrintableElementRecipe,
        cached_elements: & Vec<PrintableElementRecipe>,
    ) -> bool {
        for cached in cached_elements {
            if self.rectangles_overlap(
                element.x,
                element.y,
                element.width,
                element.height,
                cached.x,
                cached.y,
                cached.width,
                cached.height,
            ) {
                return true;
            }
        }
        false
    }

    fn rectangles_overlap(
        &self,
        x1: u32,
        y1: u32,
        w1: u32,
        h1: u32,
        x2: u32,
        y2: u32,
        w2: u32,
        h2: u32,
    ) -> bool {
        let x1_right = x1 + w1;
        let x2_right = x2 + w2;
        let y1_bottom = y1 + h1;
        let y2_bottom = y2 + h2;

        x1 < x2_right && x2 < x1_right && y1 < y2_bottom && y2 < y1_bottom
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
            let mut object_cache: Vec<PrintableElementRecipe> = Vec::new();

            image.width = self.width;
            image.height = self.height;
            image.background_path = self.pick_background()?;

            let object_count = ImageRecipeGeneratorImpl::<FS>::random(1, max_object_count_per_image);
            for _ in 0..object_count {
                let object = self.try_to_pick_object(&mut object_cache)?;
                if object.is_some() {
                    image.object.push(object.unwrap());
                }
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
