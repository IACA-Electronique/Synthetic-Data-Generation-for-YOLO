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
    fn add_scalable_object_from_file(&mut self, path: &str, x: u32, y: u32, scale: f32, angle: f32);
    fn width(&self) -> u32;
    fn height(&self) -> u32;

}


pub struct ImageEditableImage {
    image: RgbaImage,
}

impl ImageEditableImage {

    /// Rotates an RGBA image around its center without cropping its content.
    ///
    /// The image is first placed on a transparent square canvas large enough to
    /// contain the full diagonal of the original image. This prevents corners from
    /// being clipped during rotation.
    ///
    /// After rotation, the result is translated onto a smaller transparent canvas
    /// sized to fit the rotated bounding box as closely as possible.
    ///
    /// # Arguments
    ///
    /// * `object` - The source image to rotate.
    /// * `angle` - The rotation angle in degrees.
    ///
    /// # Returns
    ///
    /// A new [`RgbaImage`] containing the rotated image with a transparent
    /// background and without cropping the original content.
    ///
    /// # Notes
    ///
    /// The rotation uses bilinear interpolation, which gives smoother results than
    /// nearest-neighbor interpolation but may slightly blend edge pixels.
    fn rotate_without_crop(object: &RgbaImage, angle: f32) -> RgbaImage {
        let diagonal = ((object.width().pow(2) + object.height().pow(2)) as f32)
            .sqrt()
            .ceil() as u32;

        let mut max_sized_canvas = RgbaImage::from_pixel(
            diagonal,
            diagonal,
            Rgba([0, 0, 0, 0]),
        );

        let offset_x = (diagonal as i64 - object.width() as i64) / 2;
        let offset_y = (diagonal as i64 - object.height() as i64) / 2;

        imageops::overlay(
            &mut max_sized_canvas,
            object,
            offset_x,
            offset_y,
        );

        let angle_rad = angle.to_radians();

        let rotated_image_in_max_sized_canvas = rotate_about_center(
            &max_sized_canvas,
            angle_rad,
            Interpolation::Bilinear,
            Rgba([0, 0, 0, 0]),
        );

        let cos = angle_rad.cos().abs();
        let sin = angle_rad.sin().abs();

        let optimized_width = (
            object.width() as f32 * cos +
                object.height() as f32 * sin
        ).ceil().max(1.0) as u32;

        let optimized_height = (
            object.width() as f32 * sin +
                object.height() as f32 * cos
        ).ceil().max(1.0) as u32;

        let crop_x = (rotated_image_in_max_sized_canvas.width() - optimized_width) / 2;
        let crop_y = (rotated_image_in_max_sized_canvas.height() - optimized_height) / 2;

        imageops::crop_imm(
            &rotated_image_in_max_sized_canvas,
            crop_x,
            crop_y,
            optimized_width,
            optimized_height,
        ).to_image()
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

    fn add_scalable_object_from_file(&mut self, path: &str, x: u32, y: u32, scale: f32, angle: f32) {
        assert!(scale > 0.0, "scale must be greater than 0");

        let object = image::open(path)
            .expect("failed to open object image")
            .to_rgba8();

        let scaled_width = ((object.width() as f32) * scale).round().max(1.0) as u32;
        let scaled_height = ((object.height() as f32) * scale).round().max(1.0) as u32;

        self.add_object_from_file(path, x, y, scaled_width, scaled_height, angle)
    }

    fn width(&self) -> u32 {
        self.image.width()
    }

    fn height(&self) -> u32 {
        self.image.height()
    }
}