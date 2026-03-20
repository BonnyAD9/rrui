use minlin::Vec2;
use wgpu::{Adapter, Device, Queue, TextureFormat, TextureView};

pub trait Renderer {
    type Config;

    fn create(
        config: Self::Config,
        adapter: &Adapter,
        device: &Device,
        queue: Queue,
        format: TextureFormat,
    ) -> Self;
    fn render(
        &mut self,
        size: Vec2<u32>,
        view: &TextureView,
        format: TextureFormat,
    );
}
