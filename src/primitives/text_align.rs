use crate::Align;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub enum TextAlign {
    #[default]
    Default,
    Left,
    Center,
    Right,
    Justified,
}

impl From<Align> for TextAlign {
    fn from(value: Align) -> Self {
        match value {
            Align::Start => Self::Left,
            Align::Center => Self::Center,
            Align::End => Self::Right,
        }
    }
}
