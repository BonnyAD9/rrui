#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThumbState {
    Normal,
    Hover,
    TrackHover,
    Dragging(f32),
}
