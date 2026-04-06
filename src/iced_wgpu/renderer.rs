use iced_wgpu::{
    Engine,
    core::{
        Border, Color, Font, Point, Rectangle, Renderer as _, Shadow,
        image::{self, Renderer as _},
        renderer::Quad,
        text::{Paragraph as _, Renderer as _},
    },
    graphics::{Antialiasing, Shell, Viewport, text::Paragraph},
};
use minlin::{MapExt, Rect, Vec2};

use crate::iced_wgpu::rect;

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

impl crate::TextRenderer for Renderer {
    type Font = Font;
    type LayedText = Paragraph;

    fn default_font(&self) -> Self::Font {
        self.renderer.default_font()
    }

    fn default_font_size(&self) -> f32 {
        self.renderer.default_size().0
    }

    fn draw_clipped_text(
        &mut self,
        text: &Self::LayedText,
        pos: impl Into<Vec2<f32>>,
        fg: impl Into<crate::Color>,
        clip_bounds: impl Into<Rect<f32>>,
    ) {
        let p = pos.into();
        let f = fg.into();
        let c = clip_bounds.into();
        self.renderer.fill_paragraph(
            text,
            Point::new(p.x, p.y),
            Color::from_rgba(f.x, f.y, f.z, f.w),
            Rectangle {
                x: c.x,
                y: c.y,
                width: c.z,
                height: c.w,
            },
        );
    }

    fn draw_text(
        &mut self,
        text: &Self::LayedText,
        pos: impl Into<Vec2<f32>>,
        fg: impl Into<crate::Color>,
    ) {
        let p = pos.into();
        let f = fg.into();
        let s = text.bounds();
        self.renderer.fill_paragraph(
            text,
            Point::new(p.x, p.y),
            Color::from_rgba(f.x, f.y, f.z, f.w),
            Rectangle {
                x: p.x,
                y: p.y,
                width: s.width,
                height: s.height,
            },
        );
    }
}

impl crate::LayerRenderer for Renderer {
    fn with_clip<T>(
        &mut self,
        bounds: impl Into<Rect<f32>>,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T {
        self.renderer.start_layer(rect(bounds.into()));
        let res = f(self);
        self.renderer.end_layer();
        res
    }
}

impl crate::ImageRenderer for Renderer {
    type ImageData = image::Handle;
    type LoadedImage = image::Allocation;
    type LoadImageError = image::Error;

    fn load_image(
        &self,
        data: &Self::ImageData,
    ) -> Result<Self::LoadedImage, Self::LoadImageError> {
        self.renderer.load_image(data)
    }

    fn image_size(&self, img: &Self::ImageData) -> Vec2<u32> {
        // TODO: use faster way than load_image
        self.renderer
            .measure_image(img)
            .or_else(|| {
                self.renderer
                    .load_image(img)
                    .inspect_err(|e| eprintln!("{e}"))
                    .ok()
                    .map(|a| a.size())
            })
            .map(|a| Vec2::new(a.width, a.height))
            .unwrap_or_default()
    }

    fn draw_loaded_image_clipped(
        &mut self,
        bounds: impl Into<Rect<f32>>,
        clip_bounds: impl Into<Rect<f32>>,
        image: &Self::LoadedImage,
        params: &crate::ImageParameters,
    ) {
        self.renderer.draw_image(
            super::image(image.handle(), params),
            rect(bounds.into()),
            rect(clip_bounds.into()),
        );
    }

    fn draw_image_clipped(
        &mut self,
        bounds: impl Into<Rect<f32>>,
        clip_bounds: impl Into<Rect<f32>>,
        data: &Self::ImageData,
        params: &crate::ImageParameters,
    ) {
        self.renderer.draw_image(
            super::image(data, params),
            rect(bounds.into()),
            rect(clip_bounds.into()),
        );
    }
}
