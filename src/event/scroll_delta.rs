use minlin::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrollDelta {
    Lines(Vec2<f32>),
    Pixels(Vec2<f32>),
}
