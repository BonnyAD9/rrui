use minlin::Vec2;
use winit::event::MouseScrollDelta;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrollDelta {
    Lines(Vec2<f32>),
    Pixels(Vec2<f32>),
}

impl From<MouseScrollDelta> for ScrollDelta {
    fn from(value: MouseScrollDelta) -> Self {
        match value {
            MouseScrollDelta::LineDelta(x, y) => Self::Lines(Vec2::new(x, y)),
            MouseScrollDelta::PixelDelta(o) => {
                Self::Pixels(Vec2::new(o.x, o.y).cast())
            }
        }
    }
}
