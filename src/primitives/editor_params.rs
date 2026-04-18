use crate::{Size, TextWrap};

#[derive(Debug, Clone)]
pub struct EditorParams<F> {
    pub font: F,
    pub font_size: f32,
    pub line_height: Size,
    pub wrapping: TextWrap,
}
