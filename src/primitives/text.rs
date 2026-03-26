use std::borrow::Cow;

use minlin::Vec2;

use crate::{Align, Size, TextAlign, TextWrap};

#[derive(Debug, Clone)]
pub struct Text<'a, Font> {
    pub text: Cow<'a, str>,
    pub font: Font,
    pub font_size: f32,
    pub bounds: Vec2<f32>,
    pub align_x: TextAlign,
    pub align_y: Align,
    pub line_height: Size,
    pub wrapping: TextWrap,
}

impl<'a, Font> Text<'a, Font> {
    pub const DEFAULT_LINE_HEIGHT: Size = Size::Relative(1.3);

    pub fn new(
        text: impl Into<Cow<'a, str>>,
        font: Font,
        font_size: f32,
        bounds: impl Into<Vec2<f32>>,
    ) -> Self {
        Self {
            text: text.into(),
            font,
            bounds: bounds.into(),
            font_size,
            align_x: TextAlign::Default,
            align_y: Align::Center,
            line_height: Self::DEFAULT_LINE_HEIGHT,
            wrapping: TextWrap::None,
        }
    }
}
