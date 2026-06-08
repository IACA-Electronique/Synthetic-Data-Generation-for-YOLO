use crate::infrastructure::editable_image::EditableImage;
use crate::infrastructure::filesystem::FileSystem;
use rand::Rng;

pub trait ImageGenerator {
    fn generate<I: EditableImage>(
        &self,
        output_path: &str,
    ) -> Result<(), String>;
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

    fn add_random_object<I: EditableImage>(&self, image: &mut I, path: &str, rng: &mut impl Rng) {
        let width = image.width();
        let height = image.height();
        
        let x = if width > 0 { rng.gen_range(0..width) } else { 0 };
        let y = if height > 0 { rng.gen_range(0..height) } else { 0 };
        
        let scale = rng.gen_range(0.1..1.0);
        let angle = rng.gen_range(0.0..360.0);
        image.add_scalable_object_from_file(path, x, y, scale, angle);
    }
}

impl ImageGenerator for ImageGeneratorImpl {
    fn generate<I: EditableImage>(&self, output_path: &str) -> Result<(), String> {
        let mut rng = rand::thread_rng();

        let backgrounds = self.filesystem.list_files(&self.background_dir)?;
        if backgrounds.is_empty() {
            return Err(format!("No background images found in {}", self.background_dir));
        }
        let background_path = &backgrounds[rng.gen_range(0..backgrounds.len())];
        let mut image = I::from_file(background_path);

        let num_objects = rng.gen_range(0..=5);
        if num_objects > 0 {
            let all_objects = self.get_all_files_in_subdirs(&self.object_dir)?;
            if all_objects.is_empty() {
                return Err(format!("No object images found in {}", self.object_dir));
            }
            for _ in 0..num_objects {
                let object_path = &all_objects[rng.gen_range(0..all_objects.len())];
                self.add_random_object(&mut image, object_path, &mut rng);
            }
        }

        if let Some(dir) = &self.distraction_dir {
            let distractions = self.filesystem.list_files(dir)?;
            if !distractions.is_empty() {
                let num_distractions = rng.gen_range(0..=3);
                for _ in 0..num_distractions {
                    let distraction_path = &distractions[rng.gen_range(0..distractions.len())];
                    self.add_random_object(&mut image, distraction_path, &mut rng);
                }
            }
        }

        image.save(output_path);
        Ok(())
    }
}