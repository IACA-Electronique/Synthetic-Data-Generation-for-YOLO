use image::{imageops, Rgba, RgbaImage};
use imageproc::geometric_transformations::{rotate_about_center, Interpolation};
use mockall::automock;

#[automock]
pub trait EditableImage {
    fn from_file(path: &str) -> Self;
    fn from_nothing(width: u32, height: u32) -> Self;
    fn save(&self, path: &str);
    fn set_background_from_file(&mut self, path: &str);
    fn set_background_from_color(&mut self, color: (u8, u8, u8));
    fn add_object_from_file(&mut self, path: &str, x: u32, y: u32, width: u32, height: u32, angle: f32);
    fn width(&self) -> u32;
    fn height(&self) -> u32;

}


pub struct ImageEditableImage {
    image: RgbaImage,
}

impl ImageEditableImage {
    fn rotate_without_crop(object: &RgbaImage, angle: f32) -> RgbaImage {
        let angle_rad = angle.to_radians();

        let cos = angle_rad.cos().abs();
        let sin = angle_rad.sin().abs();

        let width = object.width() as f32;
        let height = object.height() as f32;

        // Bounding box of the rotated image. Must match
        // `top_left_and_angle_to_four_points_v2` so labels stay in sync.
        let bbox_width = (width * cos + height * sin).ceil().max(1.0) as u32;
        let bbox_height = (width * sin + height * cos).ceil().max(1.0) as u32;

        // Place the original image centered on a canvas sized to the rotated
        // bounding box, so rotation about the center never clips any content.
        let mut canvas = RgbaImage::from_pixel(
            bbox_width,
            bbox_height,
            Rgba([0, 0, 0, 0]),
        );

        let offset_x = (bbox_width as i64 - object.width() as i64) / 2;
        let offset_y = (bbox_height as i64 - object.height() as i64) / 2;

        imageops::overlay(&mut canvas, object, offset_x, offset_y);

        rotate_about_center(
            &canvas,
            angle_rad,
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
            .to_rgba8();

        let object = imageops::resize(
            &object,
            width,
            height,
            imageops::FilterType::Nearest,
        );

        let rotated = Self::rotate_without_crop(&object, angle);

        imageops::overlay(
            &mut self.image,
            &rotated,
            x.into(),
            y.into(),
        );
    }

    fn width(&self) -> u32 {
        self.image.width()
    }

    fn height(&self) -> u32 {
        self.image.height()
    }
}