use crate::Color;

#[derive(Debug, Clone, Default)]
pub enum Background {
    #[default]
    None,
    Solid(Color),
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
