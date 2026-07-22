use clap::Parser;
use std::path::Path;
use std::process::ExitCode;
use std::time::Instant;
use synthetic_data_generator_for_yolo::infrastructure::builders::editable_image_builder::EditableImageBuilderImpl;
use synthetic_data_generator_for_yolo::infrastructure::filesystem::SimpleFileSystem;
use synthetic_data_generator_for_yolo::models::dataset_config::YOLOObbDatasetConfig;
use synthetic_data_generator_for_yolo::services::data_generator_orchestrator::{DataGeneratorOrchestrator, GenerateImagesProgress, MultiThreadDataGeneratorOrchestrator};
use synthetic_data_generator_for_yolo::services::dataset_directory_structure_generator::{DatasetDirectoryStructureGenerator, DatasetDirectoryStructureGeneratorImpl};
use synthetic_data_generator_for_yolo::services::dataset_yaml_generator::{DatasetYamlGenerator, DatasetYamlGeneratorImpl};
use synthetic_data_generator_for_yolo::services::image_generator::ImageGeneratorImpl;
use synthetic_data_generator_for_yolo::services::image_recipe_generator::ImageRecipeGeneratorImpl;
use synthetic_data_generator_for_yolo::services::label_generator::ObbYoloV11LabelGenerator;
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

    #[arg(long, default_value = "3")]
    max_object_count_per_image: u32,

    #[arg(long, default_value = "2")]
    max_distraction_count_per_image: u32,

    #[arg(long, default_value = "80")]
    train_ratio: usize,

    #[arg(long, default_value = "10")]
    val_ratio: usize,

    #[arg(long, default_value = "10")]
    test_ratio: usize,

    #[arg(long, short = 'j', default_value = "1")]
    thread_count: usize,
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
            1024
        );

        let dataset_config = YOLOObbDatasetConfig::new(self.args.output_dir.clone());

        let dir_gen_structure = DatasetDirectoryStructureGeneratorImpl::new(&dataset_config, &file_system);
        dir_gen_structure.generate_structure()
            .map_err(|e| format!("Failed to generate dataset directory structure ({}): {}",self.args.output_dir, e))?;

        let yaml_generator = DatasetYamlGeneratorImpl::new(&dataset_config, &file_system);
        let yaml_filepath = yaml_generator.generate_yaml()?;
        println!("Dataset YAML file generated at {}", yaml_filepath);

        let output_dir = Path::new(&self.args.output_dir);
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir).map_err(|e| e.to_string())?;
        }

        let image_generator = ImageGeneratorImpl::<EditableImageBuilderImpl>::new();
        let label_generator = ObbYoloV11LabelGenerator::new(
            &file_system
        );
        println!("Generating {} images...", self.args.count.unwrap());
        let orchestrator =
            MultiThreadDataGeneratorOrchestrator::<_,_,_,_,SimpleFileSystem>::new(
                &recipes_generator,
                &image_generator,
                &label_generator,
                &dataset_config);


        let on_progress = Self::build_progress_callback();

        orchestrator.generate_images(
            self.args.count.unwrap(),
            self.args.max_object_count_per_image,
            self.args.max_distraction_count_per_image,
            self.args.train_ratio,
            self.args.val_ratio,
            self.args.test_ratio,
            self.args.thread_count,
            Some(&on_progress)).await?;

        Ok(())
    }

    fn build_progress_callback() -> fn(GenerateImagesProgress) {
        |progress: GenerateImagesProgress| {
            match progress {
                GenerateImagesProgress::Started { total } => {
                    println!("Starting generation of {} images...", total);
                }
                GenerateImagesProgress::RecipesGenerated { total } => {
                    println!("Generated {} image recipes.", total);
                }
                GenerateImagesProgress::Generating {
                    count,
                    total
                } => {
                    println!("[{}/{}] Generating...", count, total);
                }
                GenerateImagesProgress::Completed { total } => {
                    println!("Completed generation of {} images.", total);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();
    let output_dir = args.output_dir.clone();
    let image_count = args.count.unwrap();
    let app = App::new(args);
    let start = Instant::now();
    match app.run().await {
        Ok(()) => {
            let elapsed = start.elapsed();
            println!("---");
            println!("Output path   : {}", output_dir);
            println!("Images written: {}", image_count);
            println!("Elapsed time  : {:.2?}", elapsed);
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            ExitCode::FAILURE
        }
    }
}
