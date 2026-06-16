use mockall::automock;
use crate::infrastructure::filesystem::FileSystem;
use crate::models::image_recipe::ImageRecipe;
use crate::utils::geometry::{normalize_four_points, top_left_and_angle_to_four_points};

#[automock]
pub trait LabelGenerator {
    fn generate_one(&self, recipe: ImageRecipe, output_dir: String) -> Result<(), String>;
}

pub struct ObbYoloV11LabelGenerator<'a, FS: FileSystem> {
    filesystem: &'a FS
}

impl<'a, FS: FileSystem> ObbYoloV11LabelGenerator<'a, FS> {
    pub fn new(filesystem: &'a FS) -> Self {
        Self { filesystem }
    }
}

impl<'a, FS: FileSystem> LabelGenerator for ObbYoloV11LabelGenerator<'_, FS> {
    fn generate_one(&self, recipe: ImageRecipe, output_dir: String) -> Result<(), String> {
        let mut label = String::new();
        for object in recipe.object {
            let (w, h) = self.filesystem.get_image_size(&object.path)?;
            let w = w as f32 * object.size;
            let h = h as f32 * object.size;

            let (x1, y1, x2, y2, x3, y3, x4, y4) = top_left_and_angle_to_four_points(
                object.x,
                object.y,
                w as u32,
                h as u32,
                object.angle
            );

            let (x1, y1, x2, y2, x3, y3, x4, y4) =
                normalize_four_points(x1, y1, x2, y2, x3, y3, x4, y4, recipe.width, recipe.height);

            label = label + &format!("{} {} {} {} {} {} {} {} {}\n", object.class, x1, y1, x2, y2, x3, y3, x4, y4);
        }

        let output_file = format!("{}/{}.txt", output_dir, recipe.name);

        self.filesystem.write_text(&output_file, &label)
            .map_err(|e| format!("Failed to write label file ({}): {}", output_file, e))?;

        Ok(())
    }
}