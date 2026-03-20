use minlin::Vec2;

pub trait Renderer {
    type Inner;

    fn reset(&mut self, size: Vec2<u32>);

    fn inner_mut(&mut self) -> &mut Self::Inner;
}
