use std::ops::Range;

use minlin::{Rect, RectExt};

use crate::Orientation;

#[derive(Debug, Clone)]
pub struct ThumbLayout {
    pub bounds: Rect<f32>,
    pub range: Range<f32>,
    pub orientation: Orientation,
}

impl ThumbLayout {
    pub fn size(&self) -> f32 {
        self.orientation.component(self.bounds.size())
    }

    pub fn start_track_bounds(&self) -> Rect<f32> {
        match self.orientation {
            Orientation::Horizontal => Rect::new(
                self.range.start,
                self.bounds.y,
                self.bounds.x - self.range.start,
                self.bounds.height(),
            ),
            Orientation::Vertical => Rect::new(
                self.bounds.x,
                self.range.start,
                self.bounds.width(),
                self.bounds.y - self.range.start,
            ),
        }
    }

    pub fn end_track_bounds(&self) -> Rect<f32> {
        match self.orientation {
            Orientation::Horizontal => {
                let tr = self.bounds.right();
                Rect::new(
                    tr,
                    self.bounds.y,
                    self.range.end - tr + self.bounds.width(),
                    self.bounds.height(),
                )
            }
            Orientation::Vertical => {
                let tb = self.bounds.bottom();
                Rect::new(
                    self.bounds.x,
                    tb,
                    self.bounds.width(),
                    self.range.end - tb + self.bounds.height(),
                )
            }
        }
    }
}
