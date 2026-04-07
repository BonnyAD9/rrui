use minlin::Rect;

use crate::GridSpan;

#[derive(Debug, Clone, Copy)]
pub struct GridItem<W> {
    pub widget: W,
    pub pos: Rect,
    pub z: f32,
}

impl<W> GridItem<W> {
    pub fn new(pos: impl Into<GridSpan>, widget: W) -> Self {
        Self::new_z(0., pos, widget)
    }

    pub fn new_z(z: f32, pos: impl Into<GridSpan>, widget: W) -> Self {
        Self {
            pos: pos.into().0,
            widget,
            z,
        }
    }
}
