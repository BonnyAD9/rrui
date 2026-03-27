mod renderer;

use std::borrow::Cow;

use iced_wgpu::{
    core::{
        Background, Border, Color, Font, Point, Rectangle, Shadow, Size, Text,
        Vector,
        alignment::Vertical,
        border::Radius,
        renderer::Quad,
        text::{
            Alignment, Hit, LineHeight, Paragraph as ParagraphTrait, Shaping,
            Wrapping,
        },
    },
    graphics::text::Paragraph,
};
use minlin::Rect;

use crate::LayedText;

pub use self::renderer::*;

fn color(color: crate::Color) -> Color {
    Color::from_rgba(color.x, color.y, color.z, color.w)
}

fn rect(rect: Rect<f32>) -> Rectangle {
    Rectangle {
        x: rect.x,
        y: rect.y,
        width: rect.z,
        height: rect.w,
    }
}

impl From<crate::Radius> for Radius {
    fn from(value: crate::Radius) -> Self {
        Self {
            top_left: value.tl,
            top_right: value.tr,
            bottom_right: value.br,
            bottom_left: value.bl,
        }
    }
}

impl From<crate::Border> for Border {
    fn from(value: crate::Border) -> Self {
        Self {
            color: color(value.color),
            width: value.width,
            radius: value.radius.into(),
        }
    }
}

impl From<crate::Shadow> for Shadow {
    fn from(value: crate::Shadow) -> Self {
        Self {
            color: color(value.color),
            offset: Vector::new(value.offset.x, value.offset.y),
            blur_radius: value.radius,
        }
    }
}

impl From<&crate::Quad> for Quad {
    fn from(value: &crate::Quad) -> Self {
        Self {
            bounds: rect(value.bounds),
            border: value.border.into(),
            shadow: value.shadow.into(),
            snap: value.snap,
        }
    }
}

impl From<crate::Background> for Background {
    fn from(value: crate::Background) -> Self {
        match value {
            crate::Background::Solid(c) => Self::Color(color(c)),
            crate::Background::None => Self::Color(Color::TRANSPARENT),
        }
    }
}

impl LayedText<Font> for Paragraph {
    fn from_text(text: &crate::Text<Font>) -> Self {
        Paragraph::with_text(text.into())
    }

    fn font(&self) -> Font {
        <Self as ParagraphTrait>::font(self)
    }

    fn bounds(&self) -> minlin::Vec2<f32> {
        let s = <Self as ParagraphTrait>::bounds(self);
        minlin::Vec2::new(s.width, s.height)
    }

    fn set_bounds(
        &mut self,
        bounds: impl Into<minlin::Vec2<f32>>,
    ) -> &mut Self {
        let b = bounds.into();
        self.resize(Size::new(b.x, b.y));
        self
    }

    fn font_size(&self) -> f32 {
        self.size().0
    }

    fn align_x(&self) -> crate::TextAlign {
        <Self as ParagraphTrait>::align_x(self).into()
    }

    fn align_y(&self) -> crate::Align {
        <Self as ParagraphTrait>::align_y(self).into()
    }

    fn line_height(&self) -> crate::Size {
        <Self as ParagraphTrait>::line_height(self).into()
    }

    fn wrapping(&self) -> crate::TextWrap {
        <Self as ParagraphTrait>::wrapping(self).into()
    }

    fn min_bounds(&self) -> minlin::Vec2<f32> {
        let s = <Self as ParagraphTrait>::min_bounds(self);
        minlin::Vec2::new(s.width, s.height)
    }

    fn align_bounds(&self) -> minlin::Vec2<f32> {
        let s = <Self as ParagraphTrait>::min_bounds(self);
        minlin::Vec2::new(s.width, s.height)
    }

    fn hit_test(&self, pos: impl Into<minlin::Vec2<f32>>) -> Option<usize> {
        let p = pos.into();
        let h = <Self as ParagraphTrait>::hit_test(self, Point::new(p.x, p.y));
        if let Some(Hit::CharOffset(c)) = h {
            Some(c)
        } else {
            None
        }
    }

    fn create<'a>(
        text: impl Into<Cow<'a, str>>,
        font: Font,
        font_size: f32,
        bounds: impl Into<minlin::Vec2<f32>>,
    ) -> Self {
        let t = text.into();
        let b = bounds.into();
        Paragraph::with_text(Text {
            content: &t,
            bounds: Size::new(b.x, b.y),
            size: font_size.into(),
            line_height: crate::Text::<Font>::DEFAULT_LINE_HEIGHT.into(),
            font,
            align_x: Alignment::Left,
            align_y: Vertical::Center,
            shaping: Shaping::Auto,
            wrapping: Wrapping::None,
        })
    }
}

impl<'a> From<&'a crate::Text<'_, Font>> for Text<&'a str> {
    fn from(value: &'a crate::Text<Font>) -> Self {
        Self {
            content: &value.text,
            bounds: Size::new(value.bounds.x, value.bounds.y),
            size: value.font_size.into(),
            line_height: value.line_height.into(),
            font: value.font,
            align_x: value.align_x.into(),
            align_y: value.align_y.into(),
            shaping: Shaping::Auto,
            wrapping: value.wrapping.into(),
        }
    }
}

impl From<crate::Size> for LineHeight {
    fn from(value: crate::Size) -> Self {
        match value {
            crate::Size::Absolute(v) => Self::Absolute(v.into()),
            crate::Size::Relative(v) => Self::Relative(v),
        }
    }
}

impl From<LineHeight> for crate::Size {
    fn from(value: LineHeight) -> Self {
        match value {
            LineHeight::Relative(r) => Self::Relative(r),
            LineHeight::Absolute(a) => Self::Absolute(a.0),
        }
    }
}

impl From<crate::TextAlign> for Alignment {
    fn from(value: crate::TextAlign) -> Self {
        match value {
            crate::TextAlign::Default => Self::Default,
            crate::TextAlign::Left => Self::Left,
            crate::TextAlign::Center => Self::Center,
            crate::TextAlign::Right => Self::Right,
            crate::TextAlign::Justified => Self::Justified,
        }
    }
}

impl From<Alignment> for crate::TextAlign {
    fn from(value: Alignment) -> Self {
        match value {
            Alignment::Default => Self::Default,
            Alignment::Left => Self::Left,
            Alignment::Center => Self::Center,
            Alignment::Right => Self::Right,
            Alignment::Justified => Self::Justified,
        }
    }
}

impl From<crate::Align> for Vertical {
    fn from(value: crate::Align) -> Self {
        match value {
            crate::Align::Start => Self::Top,
            crate::Align::Center => Self::Center,
            crate::Align::End => Self::Bottom,
        }
    }
}

impl From<Vertical> for crate::Align {
    fn from(value: Vertical) -> Self {
        match value {
            Vertical::Top => Self::Start,
            Vertical::Center => Self::Center,
            Vertical::Bottom => Self::End,
        }
    }
}

impl From<crate::TextWrap> for Wrapping {
    fn from(value: crate::TextWrap) -> Self {
        match value {
            crate::TextWrap::None => Self::None,
            crate::TextWrap::Word => Self::Word,
            crate::TextWrap::Glyph => Self::Glyph,
            crate::TextWrap::WordOrGlyph => Self::WordOrGlyph,
        }
    }
}

impl From<Wrapping> for crate::TextWrap {
    fn from(value: Wrapping) -> Self {
        match value {
            Wrapping::None => Self::None,
            Wrapping::Word => Self::Word,
            Wrapping::Glyph => Self::Glyph,
            Wrapping::WordOrGlyph => Self::WordOrGlyph,
        }
    }
}

impl crate::Font for Font {}
