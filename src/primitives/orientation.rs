use minlin::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    pub fn component<T>(&self, v: impl Into<Vec2<T>>) -> T {
        let v = v.into();
        match self {
            Orientation::Horizontal => v.x,
            Orientation::Vertical => v.y,
        }
    }
}
