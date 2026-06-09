#[derive(Debug, Clone, PartialEq)]
pub struct ImageRecipe {
    pub background_path: String,
    pub distraction: Option<Vec<PrintableElementRecipe>>,
    pub object: Vec<PrintableElementRecipe>,
    pub output_path: String,
    pub width: u32,
    pub height: u32,   
}

impl ImageRecipe {
    pub fn new() -> Self {
        Self {
            background_path: String::new(),
            distraction: None,
            object: Vec::new(),
            output_path: String::new(),
            width: 0,
            height: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrintableElementRecipe {
    path: String,
    id: u32,
    size: f32,
    angle: f32,
    x: u32,
    y: u32,
}