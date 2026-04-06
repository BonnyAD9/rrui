mod layer_renderer;
mod image_renderer;
mod quad_renderer;
mod text_renderer;

pub use self::{
    layer_renderer::*, image_renderer::*, quad_renderer::*, text_renderer::*,
};

use minlin::Vec2;

pub trait Renderer {
    type Inner;

    fn reset(&mut self, size: Vec2<u32>);

    fn inner_mut(&mut self) -> &mut Self::Inner;
}
