use minlin::{Rect, Vec2};

use crate::{ImageData, ImageParameters, LoadedImage};

pub trait ImageRenderer {
    type ImageData: ImageData;
    type LoadedImage: LoadedImage;
    type LoadImageError;

    fn load_image(
        &self,
        data: &Self::ImageData,
    ) -> Result<Self::LoadedImage, Self::LoadImageError>;

    fn image_size(&self, img: &Self::ImageData) -> Vec2<u32>;

    fn draw_loaded_image_clipped(
        &mut self,
        bounds: impl Into<Rect<f32>>,
        clip_bounds: impl Into<Rect<f32>>,
        image: &Self::LoadedImage,
        params: &ImageParameters,
    );

    fn draw_image_clipped(
        &mut self,
        bounds: impl Into<Rect<f32>>,
        clip_bounds: impl Into<Rect<f32>>,
        data: &Self::ImageData,
        params: &ImageParameters,
    );

    fn draw_loaded_image(
        &mut self,
        bounds: impl Into<Rect<f32>>,
        image: &Self::LoadedImage,
        params: &ImageParameters,
    ) {
        let bounds = bounds.into();
        self.draw_loaded_image_clipped(bounds, bounds, image, params);
    }

    fn draw_image(
        &mut self,
        bounds: impl Into<Rect<f32>>,
        data: &Self::ImageData,
        params: &ImageParameters,
    ) {
        let bounds = bounds.into();
        self.draw_image_clipped(bounds, bounds, data, params);
    }
}
