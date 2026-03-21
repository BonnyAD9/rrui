use minlin::Vec2;
use wgpu::SurfaceTarget;

pub trait Window {
    fn inner_size(&self) -> Vec2<u32>;
    fn get_target(&self) -> SurfaceTarget<'static>;
    fn request_redraw(&self);
}
