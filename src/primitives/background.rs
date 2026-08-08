use minlin::{CompArithm, Rgba};

use crate::Color;

#[derive(Debug, Clone, Default)]
pub enum Background {
    #[default]
    None,
    Solid(Color),
}

impl Background {
    pub fn tint(self, strength: f32, color: Color) -> Self {
        match self {
            Background::None => Background::None,
            Background::Solid(rgba) => Background::Solid(Rgba(
                rgba.cjoin(*color, |a, b| a * (1. - strength) + b * strength),
            )),
        }
    }
}

impl<'a> From<&'a Background> for Background {
    fn from(value: &'a Background) -> Self {
        value.clone()
    }
}

impl From<Color> for Background {
    fn from(value: Color) -> Self {
        Self::Solid(value)
    }
}
