use std::borrow::Cow;

use minlin::{Rect, RectExt, Vec2};

use crate::{Color, Editor, Font, LayedText, Text};

pub trait TextRenderer {
    type Font: Font;
    type LayedText: LayedText<Self::Font>;
    type Editor: Editor;

    fn default_font(&self) -> Self::Font;

    fn default_font_size(&self) -> f32;

    fn draw_clipped_text(
        &mut self,
        text: &Self::LayedText,
        pos: impl Into<Vec2<f32>>,
        fg: impl Into<Color>,
        clip_bounds: impl Into<Rect<f32>>,
    );

    fn draw_text(
        &mut self,
        text: &Self::LayedText,
        pos: impl Into<Vec2<f32>>,
        fg: impl Into<Color>,
    ) {
        let p = pos.into();
        self.draw_clipped_text(
            text,
            p,
            fg,
            Rect::from_pos_size(p, text.bounds()),
        );
    }

    fn default_text<'a>(
        &self,
        text: impl Into<Cow<'a, str>>,
        bounds: impl Into<Vec2<f32>>,
    ) -> Text<'a, Self::Font> {
        Text::new(text, self.default_font(), self.default_font_size(), bounds)
    }

    fn lay_text(
        &self,
        text: &str,
        bounds: impl Into<Vec2<f32>>,
    ) -> Self::LayedText {
        Self::LayedText::create(
            text,
            self.default_font(),
            self.default_font_size(),
            bounds,
        )
    }

    fn draw_editor(
        &mut self,
        editor: &Self::Editor,
        pos: impl Into<Vec2<f32>>,
        fg: impl Into<Color>,
        clip_bounds: impl Into<Rect<f32>>,
    );
}
