use crate::{Color, Radius};

#[derive(Default, Debug, Clone, Copy)]
pub struct Border {
    pub color: Color,
    pub width: f32,
    pub radius: Radius,
}

impl Border {
    pub fn new(
        color: impl Into<Color>,
        width: f32,
        radius: impl Into<Radius>,
    ) -> Self {
        Self {
            color: color.into(),
            width,
            radius: radius.into(),
        }
    }

    pub fn none() -> Self {
        Self::default()
    }

    pub fn round(radius: impl Into<Radius>) -> Self {
        Self::new(Color::BLACK, 0., radius)
    }

    pub fn square(color: impl Into<Color>, width: f32) -> Self {
        Self::new(color, width, Radius::square())
    }

    pub fn px1(color: impl Into<Color>) -> Self {
        Self::square(color, 1.)
    }

    pub fn px2(color: impl Into<Color>) -> Self {
        Self::square(color, 1.)
    }

    pub fn with_radius(mut self, radius: impl Into<Radius>) -> Self {
        self.radius = radius.into();
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn with_color(mut self, color: impl Into<Color>) -> Self {
        self.color = color.into();
        self
    }
}

impl<'a> From<&'a Border> for Border {
    fn from(value: &'a Border) -> Self {
        *value
    }
}
