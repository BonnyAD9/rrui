use iced_wgpu::{
    core::{Font, Rectangle, Renderer as _},
    graphics::{Antialiasing, Shell, Viewport},
    Engine,
};
use minlin::{Vec2, Vec4};

#[derive(Debug)]
pub struct RendererConfig {
    pub font: Font,
    pub font_size: f32,
    pub scale: f32,
    pub antialiasing: bool,
    pub clear_color: Option<Vec4<f32>>,
}

pub struct Renderer {
    renderer: iced_wgpu::Renderer,
    viewport: Viewport,
    clear_color: Option<iced_wgpu::core::Color>,
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
            .map(|c| iced_wgpu::core::Color::from_rgba(c.x, c.y, c.z, c.w));

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

impl crate::Renderer for crate::iced_wgpu::Renderer {
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
            clear_color: Some(Vec4::new(0., 0., 0., 1.)),
        }
    }
}
