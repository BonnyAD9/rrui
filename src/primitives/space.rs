use crate::Size;

#[derive(Debug, Clone, Copy)]
pub enum Space {
    Relative(f32),
    Absolute(f32),
    Auto,
}

impl From<Size> for Space {
    fn from(value: Size) -> Self {
        match value {
            Size::Relative(v) => Self::Relative(v),
            Size::Absolute(v) => Self::Absolute(v),
        }
    }
}
