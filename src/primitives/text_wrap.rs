#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextWrap {
    #[default]
    None,
    Word,
    Glyph,
    WordOrGlyph,
}

impl From<bool> for TextWrap {
    fn from(value: bool) -> Self {
        if value { Self::WordOrGlyph } else { Self::None }
    }
}
