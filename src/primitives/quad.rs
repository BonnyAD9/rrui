use minlin::Rect;

use crate::{Border, Radius, Shadow};

#[derive(Debug, Clone, Copy)]
pub struct Quad {
    pub bounds: Rect<f32>,
    pub border: Border,
    pub shadow: Shadow,
    pub snap: bool,
}

impl Quad {
    pub fn new(
        bounds: impl Into<Rect<f32>>,
        border: impl Into<Border>,
        shadow: impl Into<Shadow>,
        snap: bool,
    ) -> Self {
        Self {
            bounds: bounds.into(),
            border: border.into(),
            shadow: shadow.into(),
            snap,
        }
    }

    pub fn rectangle(bounds: impl Into<Rect<f32>>) -> Self {
        Self::new(bounds, Border::none(), Shadow::none(), false)
    }

    pub fn round(
        bounds: impl Into<Rect<f32>>,
        radius: impl Into<Radius>,
    ) -> Self {
        Self::new(bounds, Border::round(radius), Shadow::none(), false)
    }

    pub fn border(
        bounds: impl Into<Rect<f32>>,
        border: impl Into<Border>,
    ) -> Self {
        Self::new(bounds, border, Shadow::none(), false)
    }

    pub fn with_radius(mut self, radius: impl Into<Radius>) -> Self {
        self.border.radius = radius.into();
        self
    }

    pub fn with_border(mut self, border: impl Into<Border>) -> Self {
        self.border = border.into();
        self
    }

    pub fn with_shadow(mut self, shadow: impl Into<Shadow>) -> Self {
        self.shadow = shadow.into();
        self
    }

    pub fn snapped(mut self) -> Self {
        self.snap = true;
        self
    }
}

impl From<Rect<f32>> for Quad {
    fn from(value: Rect<f32>) -> Self {
        Self::rectangle(value)
    }
}
