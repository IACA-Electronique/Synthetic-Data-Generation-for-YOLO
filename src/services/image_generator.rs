use std::marker::PhantomData;
use mockall::automock;
use crate::infrastructure::builders::editable_image_builder::EditableImageBuilder;
use crate::infrastructure::editable_image::EditableImage;
use crate::models::image_recipe::ImageRecipe;

#[automock]
pub trait ImageGenerator {
    fn generate(&self, recipes: Vec<ImageRecipe>, output_dir: String) -> Result<(), String>;
    fn generate_one(&self, recipe: ImageRecipe, output_dir: String) -> Result<(), String>;
}

pub struct ImageGeneratorImpl<B: EditableImageBuilder> {
    _builder: PhantomData<B>,
}

impl<B: EditableImageBuilder> ImageGeneratorImpl<B> {
    pub fn new() -> Self {
        Self {
            _builder: PhantomData,
        }
    }
}
impl<B: EditableImageBuilder> ImageGenerator for ImageGeneratorImpl<B> {
    fn generate(&self, recipes: Vec<ImageRecipe>, output_dir: String) -> Result<(), String> {
        for recipe in recipes {
            self.generate_one(recipe, output_dir.clone())?;
        }

        Ok(())
    }

    fn generate_one(&self, recipe: ImageRecipe, output_dir: String) -> Result<(), String> {
        let mut image= B::build_from_nothing(recipe.width, recipe.height);
        image.set_background_from_file(&recipe.background_path);

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

        for object in recipe.object {
            image.add_scalable_object_from_file(
                &object.path,
                object.x,
                object.y,
                object.size,
                object.angle
            )
        }

        image.save(&format!("{}/{}.png", output_dir, recipe.name));

        Ok(())
    }
}
