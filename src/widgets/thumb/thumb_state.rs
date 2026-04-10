#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThumbState {
    Normal,
    Hover,
    Dragging(f32),
}
