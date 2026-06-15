use clap::Parser;
use std::path::Path;
use std::process::ExitCode;
use synthetic_data_generator_for_yolo::infrastructure::builders::editable_image_builder::EditableImageBuilderImpl;
use synthetic_data_generator_for_yolo::infrastructure::editable_image::ImageEditableImage;
use synthetic_data_generator_for_yolo::infrastructure::filesystem::SimpleFileSystem;
use synthetic_data_generator_for_yolo::models::dataset_config::YOLOObbDatasetConfig;
use synthetic_data_generator_for_yolo::services::dataset_directory_structure_generator::{DatasetDirectoryStructureGenerator, DatasetDirectoryStructureGeneratorImpl};
use synthetic_data_generator_for_yolo::services::dataset_yaml_generator::{DatasetYamlGenerator, DatasetYamlGeneratorImpl};
use synthetic_data_generator_for_yolo::services::image_generator::{ImageGenerator, ImageGeneratorImpl};
use synthetic_data_generator_for_yolo::services::image_recipe_generator::{ImageRecipeGenerator, ImageRecipeGeneratorImpl};
use synthetic_data_generator_for_yolo::settings::VERSION;

#[derive(Parser)]
#[command(version = VERSION, about = "Synthetic data generator for YOLO")]
struct Args {
    /// Path to background images
    #[arg(long)]
    background_dir: String,

    /// Path to object images (organized in subfolders by category)
    #[arg(long)]
    object_dir: String,

    /// Optional path to distraction images
    #[arg(long)]
    distraction_dir: Option<String>,

    /// Path to output directory
    #[arg(long)]
    output_dir: String,

    #[arg(long, short = 'c', required = true)]
    count: Option<u32>,

    #[arg(long, default_value = "80")]
    train_ratio: usize,

    #[arg(long, default_value = "10")]
    val_ratio: usize,

    #[arg(long, default_value = "10")]
    test_ratio: usize,
}

struct App {
    args: Args,
}

impl App {
    pub fn new(args: Args) -> Self {
        Self { args }
    }

    async fn run(&self) -> Result<(), String> {
        let file_system = SimpleFileSystem::new();

        let recipes_generator = ImageRecipeGeneratorImpl::new(
            &file_system,
            self.args.background_dir.clone(),
            self.args.object_dir.clone(),
            self.args.distraction_dir.clone(),
            1024,
            1024,
            self.args.output_dir.clone(),
        );

        let dataset_config = YOLOObbDatasetConfig::new(self.args.output_dir.clone());

        let dir_gen_structure = DatasetDirectoryStructureGeneratorImpl::new(&dataset_config, &file_system);
        dir_gen_structure.generate_structure()
            .map_err(|e| format!("Failed to generate dataset directory structure ({}): {}",self.args.output_dir, e))?;

        let yaml_generator = DatasetYamlGeneratorImpl::new(&dataset_config, &file_system);
        let yaml_filepath = yaml_generator.generate_yaml()?;
        println!("Dataset YAML file generated at {}", yaml_filepath);

        println!("Generating {} recipes images...", self.args.count.unwrap());
        let recipes = recipes_generator.generate(self.args.count.unwrap())?;

        let output_dir = Path::new(&self.args.output_dir);
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir).map_err(|e| e.to_string())?;
        }

        println!("Generating {} images...", self.args.count.unwrap());
        let generator = ImageGeneratorImpl::<EditableImageBuilderImpl>::new();
        generator.generate(recipes)?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();
    let app = App::new(args);
    match app.run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {}", error);
            ExitCode::FAILURE
        }
    }
}
