use crate::infrastructure::editable_image::{EditableImage, ImageEditableImage};

pub trait EditableImageBuilder {
    type Image: EditableImage;

    fn build_from_nothing(width: u32, height: u32) -> Self::Image;
}

#[derive(Default)]
pub struct EditableImageBuilderImpl;

impl EditableImageBuilder for EditableImageBuilderImpl {
    type Image = ImageEditableImage;

    fn build_from_nothing(width: u32, height: u32) -> Self::Image {
        ImageEditableImage::from_nothing(width, height)
    }
}