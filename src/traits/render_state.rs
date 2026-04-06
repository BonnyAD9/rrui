use minlin::Vec2;

pub trait RenderState<Window, Renderer>: Sized {
    type Config;
    type Error: std::error::Error;

    fn create(
        config: Self::Config,
        window: Window,
    ) -> Result<Self, Self::Error>;

    fn resize(&mut self, size: Vec2<u32>);

    fn render(&mut self);

    fn renderer(&mut self) -> &mut Renderer;

    fn request_redraw(&mut self);
}
