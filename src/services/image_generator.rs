use mockall::automock;
use crate::infrastructure::builders::editable_image_builder::EditableImageBuilder;
use crate::infrastructure::editable_image::EditableImage;
use crate::models::image_recipe::ImageRecipe;

#[automock]
pub trait ImageGenerator {
    fn generate(&self, recipes: Vec<ImageRecipe>) -> Result<(), String>;
}

pub struct ImageGeneratorImpl<'a, B: EditableImageBuilder> {
    editable_image_builder: &'a B,
}

impl<'a, B: EditableImageBuilder> ImageGeneratorImpl<'a, B> {

    pub fn new(editable_image_builder: &'a B) -> Self {
        Self { editable_image_builder }
    }
}

impl<B: EditableImageBuilder> ImageGenerator for ImageGeneratorImpl<'_, B> {
    fn generate(&self, recipes: Vec<ImageRecipe>) -> Result<(), String> {
        for recipe in recipes {
            let mut image= B::build_from_nothing(recipe.width, recipe.height);
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
