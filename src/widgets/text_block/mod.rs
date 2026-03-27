mod text_block_theme;

pub use self::text_block_theme::*;

use std::borrow::Cow;

use minlin::{Rect, RectExt, Vec2};

use crate::{
    Align, Element, LayedText, Size, Text, TextAlign, TextRenderer, TextWrap,
    Widget, WidgetExt, event::EventInfo,
};

#[derive(Debug, Clone)]
pub struct TextBlock<Style, Font, LText> {
    pub text: Cow<'static, str>,
    pub font: Option<Font>,
    pub font_size: Option<f32>,
    pub align_x: TextAlign,
    pub align_y: Align,
    pub line_height: Size,
    pub wrapping: TextWrap,
    pub size: Option<Vec2<f32>>,
    pub style: Style,
    pos: Vec2<f32>,
    bounds: Rect<f32>,
    layed: Option<LText>,
}

impl<Style, Font, LText> TextBlock<Style, Font, LText> {
    pub fn styled(style: Style, text: impl Into<Cow<'static, str>>) -> Self {
        Self {
            text: text.into(),
            font: None,
            font_size: None,
            align_x: TextAlign::Left,
            align_y: Align::Center,
            line_height: Text::<Font>::DEFAULT_LINE_HEIGHT,
            wrapping: TextWrap::None,
            size: None,
            style,
            pos: Vec2::default(),
            bounds: Rect::default(),
            layed: None,
        }
    }

    pub fn new(text: impl Into<Cow<'static, str>>) -> Self
    where
        Style: Default,
    {
        Self::styled(Style::default(), text)
    }
}

impl<Style: Default, Font, LText> Default for TextBlock<Style, Font, LText> {
    fn default() -> Self {
        Self::new("")
    }
}

impl<Rend, Msg, Evt, Theme, Style> Widget<Rend, Msg, Evt, Theme>
    for TextBlock<Style, Rend::Font, Rend::LayedText>
where
    Rend: TextRenderer,
    Theme: TextBlockTheme<Style = Style>,
{
    fn layout(
        &mut self,
        _: &mut crate::Shell,
        _: &Theme,
        bounds: &crate::LayoutBounds,
        renderer: &Rend,
    ) -> Rect<f32> {
        if let Some(s) = self.size {
            self.bounds = bounds.clamp(s);
            let t = if let Some(t) = &mut self.layed {
                t.set_bounds(self.bounds.size());
                t
            } else {
                let text = self.make_text(renderer, self.bounds.size());
                self.layed.insert(LayedText::from_text(&text))
            };
            let align_bounds = t.align_bounds();
            self.pos = self.get_text_pos(align_bounds);
            self.bounds
        } else {
            let b = bounds.best_max();
            let t = if let Some(t) = &mut self.layed {
                t.set_bounds(b.size());
                t
            } else {
                let text = self.make_text(renderer, b.size());
                self.layed.insert(LayedText::from_text(&text))
            };
            let min_bounds = t.min_bounds();
            self.bounds = bounds.best_at_least(min_bounds);
            t.set_bounds(self.bounds.size());
            let align_bounds = t.align_bounds();
            self.pos = self.get_text_pos(align_bounds);
            self.bounds
        }
    }

    fn size(&self, _: &Theme) -> Vec2<f32> {
        self.size.unwrap_or(Vec2::new(f32::INFINITY, f32::INFINITY))
    }

    fn reposition(&mut self, _: &Theme, pos: Vec2<f32>) {
        self.bounds.set_pos(pos);
    }

    fn event(
        &mut self,
        _: &mut crate::Shell,
        _: &Theme,
        _: &EventInfo<Evt>,
    ) -> bool {
        false
    }

    fn draw(
        &mut self,
        _: &mut crate::Shell,
        theme: &Theme,
        renderer: &mut Rend,
    ) {
        assert!(self.layed.is_some());
        let Some(text) = &self.layed else {
            unreachable!()
        };

        let fg = theme.foreground(&self.style);
        renderer.draw_clipped_text(text, self.pos, fg, self.bounds);
    }
}

impl<Style, Font: crate::Font, LText: LayedText<Font>>
    TextBlock<Style, Font, LText>
{
    fn make_text(
        &self,
        renderer: &impl TextRenderer<Font = Font, LayedText = LText>,
        bounds: Vec2<f32>,
    ) -> Text<'_, Font> {
        Text {
            text: Cow::Borrowed(&self.text),
            font: self
                .font
                .as_ref()
                .cloned()
                .unwrap_or_else(|| renderer.default_font()),
            font_size: self
                .font_size
                .unwrap_or_else(|| renderer.default_font_size()),
            bounds,
            align_x: self.align_x,
            align_y: self.align_y,
            line_height: self.line_height,
            wrapping: self.wrapping,
        }
    }

    fn get_text_pos(&self, tbounds: Vec2<f32>) -> Vec2<f32> {
        let mut pos = self.bounds.pos();

        match self.align_x {
            TextAlign::Right => {
                pos.x = self.bounds.x + self.bounds.width() - tbounds.x;
            }
            TextAlign::Center => {
                pos.x = self.bounds.x + (self.bounds.width() - tbounds.x) / 2.;
            }
            _ => {}
        }

        match self.align_y {
            Align::Start => {}
            Align::Center => {
                pos.y =
                    self.bounds.y + (self.bounds.height() - tbounds.y) / 2.;
            }
            Align::End => {
                pos.y = self.bounds.y + self.bounds.height() - tbounds.y;
            }
        }
        pos
    }
}

impl<Rend, Msg, Evt, Theme, Style>
    From<TextBlock<Style, Rend::Font, Rend::LayedText>>
    for Element<Rend, Msg, Evt, Theme>
where
    Rend: TextRenderer + 'static,
    Theme: TextBlockTheme<Style = Style>,
    Style: 'static,
{
    fn from(value: TextBlock<Style, Rend::Font, Rend::LayedText>) -> Self {
        Self::new(value)
    }
}

impl<S, Font, LText> WidgetExt for TextBlock<S, Font, LText> {}
