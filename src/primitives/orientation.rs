use std::mem;

use minlin::{Padding, Vec2};

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

    pub fn other_component<T>(&self, v: impl Into<Vec2<T>>) -> T {
        let v = v.into();
        match self {
            Orientation::Horizontal => v.y,
            Orientation::Vertical => v.x,
        }
    }

    pub fn horizontal_padding(&self, mut p: Padding) -> Padding {
        if *self == Orientation::Vertical {
            mem::swap(&mut p.0.x, &mut p.0.y);
            mem::swap(&mut p.0.z, &mut p.0.w);
        }
        p
    }

    pub fn vertical_padding(&self, mut p: Padding) -> Padding {
        if *self == Orientation::Horizontal {
            mem::swap(&mut p.0.x, &mut p.0.y);
            mem::swap(&mut p.0.z, &mut p.0.w);
        }
        p
    }
}
