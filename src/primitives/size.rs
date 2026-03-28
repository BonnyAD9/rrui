use std::ops::Add;

use minlin::Vec2;

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Relative(f32),
    Absolute(f32),
}

impl Size {
    pub fn is_absolute(&self) -> bool {
        matches!(self, Size::Absolute(_))
    }

    pub fn is_relative(&self) -> bool {
        matches!(self, Size::Relative(_))
    }

    pub fn to_parts(&self) -> Vec2<f32> {
        match self {
            Self::Absolute(v) => Vec2::new(*v, 0.),
            Self::Relative(v) => Vec2::new(0., *v),
        }
    }
}

impl Add<Size> for Size {
    type Output = Vec2<f32>;

    fn add(self, rhs: Size) -> Self::Output {
        self.to_parts() + rhs.to_parts()
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::Absolute(0.)
    }
}
