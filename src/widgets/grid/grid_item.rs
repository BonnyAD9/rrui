use minlin::Rect;

use crate::GridSpan;

#[derive(Debug, Clone, Copy)]
pub struct GridItem<W> {
    pub widget: W,
    pub pos: Rect,
}

impl<W> GridItem<W> {
    pub fn new(pos: impl Into<GridSpan>, widget: W) -> Self {
        Self {
            pos: pos.into().0,
            widget,
        }
    }
}
