use std::sync::Arc;

use minlin::Vec2;
use wgpu::SurfaceTarget;

pub trait Window {
    fn inner_size(&self) -> Vec2<u32>;
    fn get_target(&self) -> SurfaceTarget<'static>;
    fn request_redraw(&self);
}

impl Window for Arc<winit::window::Window> {
    fn inner_size(&self) -> Vec2<u32> {
        let s = winit::window::Window::inner_size(self);
        Vec2::new(s.width, s.height)
    }

    fn get_target(&self) -> SurfaceTarget<'static> {
        self.clone().into()
    }

    fn request_redraw(&self) {
        winit::window::Window::request_redraw(self);
    }
}
