use std::process::ExitCode;
use synthetic_data_generator_for_yolo::infrastructure::editable_image::{EditableImage, ImageEditableImage};
use synthetic_data_generator_for_yolo::settings::VERSION;

struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    async fn run(&self) -> Result<(), String> {

        let mut image = ImageEditableImage::from_nothing(1920, 1080);
        image.set_background_from_color((255, 255, 255));
        image.add_scalable_object_from_file("toto.png", 0,0, 1.0,0.0);
        image.save("output.png");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    println!("App version: {}", VERSION);
    let app = App::new();
    match app.run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{}", error);
            ExitCode::FAILURE
        }
    }
}
