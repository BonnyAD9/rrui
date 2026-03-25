use iced_wgpu::{
    Engine,
    core::{
        Border, Color, Font, Rectangle, Renderer as _, Shadow, renderer::Quad,
    },
    graphics::{Antialiasing, Shell, Viewport},
};
use minlin::{MapExt, Vec2};

#[derive(Debug)]
pub struct RendererConfig {
    pub font: Font,
    pub font_size: f32,
    pub scale: f32,
    pub antialiasing: bool,
    pub clear_color: Option<crate::Color>,
}

pub struct Renderer {
    renderer: iced_wgpu::Renderer,
    viewport: Viewport,
    clear_color: Option<Color>,
}

impl crate::wgpu::Renderer for Renderer {
    type Config = RendererConfig;

    fn create(
        config: Self::Config,
        adapter: &wgpu::Adapter,
        device: &wgpu::Device,
        queue: wgpu::Queue,
        format: wgpu::TextureFormat,
    ) -> Self {
        let engine = Engine::new(
            adapter,
            device.clone(),
            queue,
            format,
            config.antialiasing.then_some(Antialiasing::MSAAx4),
            Shell::headless(),
        );

        let renderer = iced_wgpu::Renderer::new(
            engine,
            config.font,
            config.font_size.into(),
        );

        let viewport =
            Viewport::with_physical_size((0, 0).into(), config.scale);
        let clear_color = config
            .clear_color
            .map(|c| Color::from_rgba(c.x, c.y, c.z, c.w));

        Self {
            renderer,
            viewport,
            clear_color,
        }
    }

    fn render(
        &mut self,
        size: Vec2<u32>,
        view: &wgpu::TextureView,
        format: wgpu::TextureFormat,
    ) {
        self.viewport = Viewport::with_physical_size(
            (size.x, size.y).into(),
            self.viewport.scale_factor(),
        );
        // let size = size.cast::<f32>();
        // self.renderer.reset(Rectangle::new((0., 0.).into(), (size.x, size.y).into()));
        self.renderer
            .present(self.clear_color, format, view, &self.viewport);
    }
}

impl crate::Renderer for Renderer {
    type Inner = iced_wgpu::Renderer;

    fn reset(&mut self, size: Vec2<u32>) {
        self.viewport = Viewport::with_physical_size(
            (size.x, size.y).into(),
            self.viewport.scale_factor(),
        );
        let size = size.cast::<f32>();
        self.renderer
            .reset(Rectangle::new((0., 0.).into(), (size.x, size.y).into()));
    }

    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.renderer
    }
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            font: Default::default(),
            font_size: 16.,
            scale: 1.,
            antialiasing: true,
            clear_color: Some(crate::Color::BLACK),
        }
    }
}

impl crate::QuadRenderer for Renderer {
    fn draw_quad(
        &mut self,
        quad: &crate::Quad,
        bg: impl Into<crate::Background>,
    ) {
        self.renderer.fill_quad(quad.into(), bg.into());
    }

    fn draw_rect(
        &mut self,
        rect: impl Into<minlin::Rect<f32>>,
        bg: impl Into<crate::Background>,
    ) {
        self.renderer.fill_quad(
            Quad {
                bounds: super::rect(rect.into()),
                border: Border::default(),
                shadow: Shadow::default(),
                snap: false,
            },
            bg.into(),
        );
    }

    fn draw_border(
        &mut self,
        bounds: impl Into<minlin::Rect<f32>>,
        border: impl Into<crate::Border>,
        bg: impl Into<crate::Background>,
    ) {
        self.renderer.fill_quad(
            Quad {
                bounds: super::rect(bounds.into()),
                border: border.into().into(),
                shadow: Shadow::default(),
                snap: false,
            },
            bg.into(),
        );
    }
}
