use minlin::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct GridItem<W> {
    pub widget: W,
    pub pos: Vec2,
}

impl<W> GridItem<W> {
    pub fn new(pos: impl Into<Vec2>, widget: W) -> Self {
        Self {
            pos: pos.into(),
            widget,
        }
    }
}
