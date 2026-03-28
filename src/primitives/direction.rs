#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Left,
    Top,
    Right,
    Bottom,
}

impl Direction {
    pub fn is_from_end(&self) -> bool {
        matches!(self, Direction::Right | Direction::Bottom)
    }
}
