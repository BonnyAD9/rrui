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

impl<Window: crate::wgpu::Window, Renderer: crate::wgpu::Renderer>
    RenderState<Window, Renderer> for crate::wgpu::State<Window, Renderer>
{
    type Config = Renderer::Config;
    type Error = crate::wgpu::StateCreateError;

    fn create(
        config: Self::Config,
        window: Window,
    ) -> Result<Self, Self::Error> {
        Self::new(window, config)
    }

    fn resize(&mut self, size: Vec2<u32>) {
        self.resize(size);
    }

    fn render(&mut self) {
        self.render();
    }

    fn renderer(&mut self) -> &mut Renderer {
        self.renderer()
    }

    fn request_redraw(&mut self) {
        self.request_redraw()
    }
}
