#[derive(Debug, Clone, PartialEq)]
pub struct ImageRecipe {
    pub name: String,
    pub background_path: String,
    pub distraction: Option<Vec<PrintableElementRecipe>>,
    pub object: Vec<PrintableElementRecipe>,
    pub width: u32,
    pub height: u32,   
}

impl ImageRecipe {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            background_path: String::new(),
            distraction: None,
            object: Vec::new(),
            width: 0,
            height: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PrintableElementRecipe {
    pub path: String,
    pub class: u32,
    pub width: u32,
    pub height: u32,
    pub angle: f32,
    pub x: u32,
    pub y: u32,
}

impl PrintableElementRecipe {
    pub fn new(path: String, id: u32, width: u32, height: u32, angle: f32, x: u32, y: u32) -> Self {
        Self {
            path,
            class: id,
            width,
            height,
            angle,
            x,
            y,
        }
    }
}