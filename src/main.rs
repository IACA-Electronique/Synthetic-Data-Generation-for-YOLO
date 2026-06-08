use std::process::ExitCode;
use synthetic_data_generator_for_yolo::settings::VERSION;

struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    async fn run(&self) -> Result<(), String> {
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
