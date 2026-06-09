use crate::infrastructure::editable_image::EditableImage;
use crate::infrastructure::filesystem::FileSystem;
use rand::Rng;

/// Maximum number of objects that can be added to a single generated image.
const MAX_OBJECTS_PER_IMAGE: u32 = 5;
/// Maximum number of distraction objects that can be added to a single generated image.
const MAX_DISTRACTIONS_PER_IMAGE: u32 = 3;

pub trait ImageGenerator {
    fn generate<I: EditableImage>(&self, output_path: &str) -> Result<(), String>;
}

pub struct ImageGeneratorImpl {
    filesystem: Box<dyn FileSystem>,
    background_dir: String,
    object_dir: String,
    distraction_dir: Option<String>,
}

impl ImageGeneratorImpl {
    pub fn new(
        filesystem: Box<dyn FileSystem>,
        background_dir: String,
        object_dir: String,
        distraction_dir: Option<String>,
    ) -> Self {
        Self {
            filesystem,
            background_dir,
            object_dir,
            distraction_dir,
        }
    }

    fn get_all_files_in_subdirs(&self, directory: &str) -> Result<Vec<String>, String> {
        let categories = self.filesystem.list_subdirectories(directory)?;
        let mut all_files = Vec::new();
        if categories.is_empty() {
            all_files = self.filesystem.list_files(directory)?;
        } else {
            for category in categories {
                let files = self.filesystem.list_files(&category)?;
                all_files.extend(files);
            }
        }
        Ok(all_files)
    }

    fn pick_background(&self, rng: &mut impl Rng) -> Result<String, String> {
        let backgrounds = self.filesystem.list_files(&self.background_dir)?;

        if backgrounds.is_empty() {
            Err(format!(
                "No background images found in {}",
                self.background_dir
            ))
        } else {
            Ok(backgrounds[rng.gen_range(0..backgrounds.len())].clone())
        }
    }

    fn add_objects<I: EditableImage>(
        &self,
        image: &mut I,
        rng: &mut impl Rng,
    ) -> Result<(), String> {
        let num_objects = rng.gen_range(0..=MAX_OBJECTS_PER_IMAGE);

        if num_objects > 0 {
            let all_objects = self.get_all_files_in_subdirs(&self.object_dir)?;
            if all_objects.is_empty() {
                return Err(format!("No object images found in {}", self.object_dir));
            }
            for _ in 0..num_objects {
                let object_path = &all_objects[rng.gen_range(0..all_objects.len())];
                self.add_random_object(image, object_path, rng);
            }
        }
        Ok(())
    }

    fn add_distractions<I: EditableImage>(
        &self,
        image: &mut I,
        rng: &mut impl Rng,
    ) -> Result<(), String> {
        if let Some(dir) = &self.distraction_dir {
            let distractions = self.filesystem.list_files(dir)?;
            if !distractions.is_empty() {
                let num_distractions = rng.gen_range(0..=MAX_DISTRACTIONS_PER_IMAGE);
                for _ in 0..num_distractions {
                    let distraction_path = &distractions[rng.gen_range(0..distractions.len())];
                    self.add_random_object(image, distraction_path, rng);
                }
            }
        }
        Ok(())
    }

    fn add_random_object<I: EditableImage>(&self, image: &mut I, path: &str, rng: &mut impl Rng) {
        let width = image.width();
        let height = image.height();

        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);

        let scale = rng.gen_range(0.1..1.0);
        let angle = rng.gen_range(0.0..360.0);

        image.add_scalable_object_from_file(path, x, y, scale, angle);
    }
}

impl ImageGenerator for ImageGeneratorImpl {
    fn generate<I: EditableImage>(&self, output_path: &str) -> Result<(), String> {
        let mut rng = rand::thread_rng();

        let background_path = self.pick_background(&mut rng)?;
        let mut image = I::from_file(&background_path);

        self.add_objects(&mut image, &mut rng)?;
        self.add_distractions(&mut image, &mut rng)?;

        image.save(output_path);
        Ok(())
    }
}
