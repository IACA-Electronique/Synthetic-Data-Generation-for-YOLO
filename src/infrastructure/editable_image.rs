use image::{imageops, Rgba, RgbaImage};
use imageproc::geometric_transformations::{rotate_about_center, Interpolation};

pub trait EditableImage {

    fn from_file(path: &str) -> Self;
    fn from_nothing(width: u32, height: u32) -> Self;
    fn save(&self, path: &str);
    fn set_background_from_file(&mut self, path: &str);
    fn set_background_from_color(&mut self, color: (u8, u8, u8));
    fn add_object_from_file(&mut self, path: &str, x: u32, y: u32, width: u32, height: u32, angle: f32);
    fn add_scalable_object_from_file(&mut self, path: &str, x: u32, y: u32, scale: f32, angle: f32);

}


pub struct ImageEditableImage {
    image: RgbaImage,
}

impl ImageEditableImage {
    fn rotate_without_crop(object: &RgbaImage, angle: f32) -> RgbaImage {
        let width = object.width();
        let height = object.height();

        let diagonal = ((width.pow(2) + height.pow(2)) as f32)
            .sqrt()
            .ceil() as u32;

        let mut canvas = RgbaImage::from_pixel(
            diagonal,
            diagonal,
            Rgba([0, 0, 0, 0]),
        );

        let offset_x = ((diagonal - width) / 2) as i64;
        let offset_y = ((diagonal - height) / 2) as i64;

        imageops::overlay(
            &mut canvas,
            object,
            offset_x,
            offset_y,
        );

        rotate_about_center(
            &canvas,
            angle.to_radians(),
            Interpolation::Bilinear,
            Rgba([0, 0, 0, 0]),
        )
    }
}

impl EditableImage for ImageEditableImage {
    fn from_file(path: &str) -> Self {
        let image = image::open(path)
            .expect("failed to open image")
            .to_rgba8();

        Self { image }
    }

    fn from_nothing(width: u32, height: u32) -> Self {
        let image = RgbaImage::from_pixel(
            width,
            height,
            Rgba([0, 0, 0, 0]),
        );

        Self { image }
    }

    fn save(&self, path: &str) {
        self.image
            .save(path)
            .expect("failed to save image");
    }

    fn set_background_from_file(&mut self, path: &str) {
        let background = image::open(path)
            .expect("failed to open background image")
            .resize_exact(
                self.image.width(),
                self.image.height(),
                imageops::FilterType::Lanczos3,
            )
            .to_rgba8();

        self.image = background;
    }

    fn set_background_from_color(&mut self, color: (u8, u8, u8)) {
        let rgba = Rgba([color.0, color.1, color.2, 255]);

        for pixel in self.image.pixels_mut() {
            *pixel = rgba;
        }
    }

    fn add_object_from_file(
        &mut self,
        path: &str,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        angle: f32,
    ) {
        let object = image::open(path)
            .expect("failed to open object image")
            .resize_exact(
                width,
                height,
                imageops::FilterType::Lanczos3,
            )
            .to_rgba8();

        let rotated = Self::rotate_without_crop(&object, angle);

        imageops::overlay(
            &mut self.image,
            &rotated,
            x.into(),
            y.into(),
        );
    }

    fn add_scalable_object_from_file(&mut self, path: &str, x: u32, y: u32, scale: f32, angle: f32) {
        assert!(scale > 0.0, "scale must be greater than 0");

        let object = image::open(path)
            .expect("failed to open object image")
            .to_rgba8();

        let scaled_width = ((object.width() as f32) * scale).round().max(1.0) as u32;
        let scaled_height = ((object.height() as f32) * scale).round().max(1.0) as u32;

        self.add_object_from_file(path, x, y, scaled_width, scaled_height, angle)
    }
}