use clap::Parser;
use std::path::Path;
use std::process::ExitCode;
use synthetic_data_generator_for_yolo::infrastructure::editable_image::ImageEditableImage;
use synthetic_data_generator_for_yolo::infrastructure::filesystem::SimpleFileSystem;
use synthetic_data_generator_for_yolo::services::image_generator::{ImageGenerator, ImageGeneratorImpl};
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
}

struct App {
    args: Args,
}

impl App {
    pub fn new(args: Args) -> Self {
        Self { args }
    }

    async fn run(&self) -> Result<(), String> {
        let filesystem = Box::new(SimpleFileSystem::new());
        let generator = ImageGeneratorImpl::new(
            filesystem,
            self.args.background_dir.clone(),
            self.args.object_dir.clone(),
            self.args.distraction_dir.clone(),
        );

        let output_dir = Path::new(&self.args.output_dir);
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir).map_err(|e| e.to_string())?;
        }

        let output_dir_str = output_dir.to_str().unwrap();
        generator.generate::<ImageEditableImage>(&format!("{}/image.png", output_dir_str))?;

        println!("Image generated successfully at {}", output_dir_str);
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
