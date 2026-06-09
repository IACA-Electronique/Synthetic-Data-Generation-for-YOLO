use crate::infrastructure::editable_image::EditableImage;
use crate::models::image_recipe::ImageRecipe;

pub trait ImageGenerator {
    fn generate<E: EditableImage>(&self, recipes: Vec<ImageRecipe>) -> Result<(), String>;
}

pub struct ImageGeneratorImpl {

}

impl ImageGeneratorImpl {

    pub fn new() -> Self {
        Self {}
    }
}

impl ImageGenerator for ImageGeneratorImpl {
    fn generate<E: EditableImage>(&self, recipes: Vec<ImageRecipe>) -> Result<(), String> {
        for recipe in recipes {
            let mut image = E::from_nothing(recipe.width, recipe.height);
            image.set_background_from_file(&recipe.background_path);

            for object in recipe.object {
                image.add_scalable_object_from_file(
                    &object.path,
                    object.x,
                    object.y,
                    object.size,
                    object.angle
                )
            }
            if let Some(distractions) = recipe.distraction {
                for distraction in distractions {
                    image.add_scalable_object_from_file(
                        &distraction.path,
                        distraction.x,
                        distraction.y,
                        distraction.size,
                        distraction.angle
                    )
                }
            }
            image.save(&recipe.output_path);
        }

        Ok(())
    }
}
