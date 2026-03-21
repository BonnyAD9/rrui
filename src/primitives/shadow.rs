use minlin::Vec2;

use crate::Color;

#[derive(Debug, Copy, Clone, Default)]
pub struct Shadow {
    pub color: Color,
    pub offset: Vec2<f32>,
    pub radius: f32,
}

impl Shadow {
    pub fn new(
        color: impl Into<Color>,
        offset: impl Into<Vec2<f32>>,
        radius: f32,
    ) -> Self {
        Self {
            color: color.into(),
            offset: offset.into(),
            radius,
        }
    }

    pub fn none() -> Self {
        Self::default()
    }

    pub fn normal() -> Self {
        Self::new(Color::BLACK, (5., 5.), 5.)
    }

    pub fn with_color(mut self, color: impl Into<Color>) -> Self {
        self.color = color.into();
        self
    }

    pub fn with_offset(mut self, offset: impl Into<Vec2<f32>>) -> Self {
        self.offset = offset.into();
        self
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}
