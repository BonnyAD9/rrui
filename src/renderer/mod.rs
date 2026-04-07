mod image_renderer;
mod layer_renderer;
mod quad_renderer;
mod svg_renderer;
mod text_renderer;

pub use self::{
    image_renderer::*, layer_renderer::*, quad_renderer::*, svg_renderer::*,
    text_renderer::*,
};

use minlin::Vec2;

pub trait Renderer {
    type Inner;

    fn reset(&mut self, size: Vec2<u32>);

    fn inner_mut(&mut self) -> &mut Self::Inner;
}
