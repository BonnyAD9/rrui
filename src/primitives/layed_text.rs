use std::borrow::Cow;

use minlin::Vec2;

use crate::{Align, Size, Text, TextAlign, TextWrap};

pub trait LayedText<Font>: Sized + 'static {
    fn from_text(text: &Text<Font>) -> Self;

    fn font(&self) -> Font;

    fn bounds(&self) -> Vec2<f32>;

    fn set_bounds(&mut self, bounds: impl Into<Vec2<f32>>) -> &mut Self;

    fn font_size(&self) -> f32;

    fn align_x(&self) -> TextAlign;

    fn align_y(&self) -> Align;

    fn line_height(&self) -> Size;

    fn wrapping(&self) -> TextWrap;

    fn min_bounds(&self) -> Vec2<f32>;

    fn hit_test(&self, pos: impl Into<Vec2<f32>>) -> Option<usize>;

    fn create<'a>(
        text: impl Into<Cow<'a, str>>,
        font: Font,
        font_size: f32,
        bounds: impl Into<Vec2<f32>>,
    ) -> Self {
        Self::from_text(&Text::new(text, font, font_size, bounds))
    }
}
