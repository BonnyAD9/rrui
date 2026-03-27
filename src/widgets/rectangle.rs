use std::fmt::Debug;

use minlin::{Rect, RectExt, Vec2};

use crate::{
    Background, Border, Element, LayoutBounds, QuadRenderer, Shell, Widget,
    WidgetExt, event::EventInfo,
};

#[derive(Debug)]
pub struct Rectangle {
    background: Background,
    border: Border,
    size: Vec2<f32>,
    bounds: Rect<f32>,
}

impl Rectangle {
    pub fn new(
        size: impl Into<Vec2<f32>>,
        bg: impl Into<Background>,
        border: impl Into<Border>,
    ) -> Self {
        Self {
            background: bg.into(),
            border: border.into(),
            size: size.into(),
            bounds: Rect::default(),
        }
    }
}

impl<Rend: QuadRenderer, Msg, Evt: Debug, Theme> Widget<Rend, Msg, Evt, Theme>
    for Rectangle
{
    fn layout(
        &mut self,
        _: &mut Shell<Msg>,
        _: &Theme,
        bounds: &LayoutBounds,
        _: &Rend,
    ) -> Rect<f32> {
        self.bounds = bounds.clamp(self.size);
        self.bounds
    }

    fn event(
        &mut self,
        _: &mut Shell<Msg>,
        _: &Theme,
        _: &EventInfo<Evt>,
    ) -> bool {
        false
    }

    fn draw(&mut self, _: &mut Shell<Msg>, _: &Theme, renderer: &mut Rend) {
        renderer.draw_border(self.bounds, self.border, &self.background);
    }

    fn size(&self, _: &Theme) -> Vec2<f32> {
        self.size
    }

    fn reposition(&mut self, _: &Theme, pos: Vec2<f32>) {
        self.bounds.set_pos(pos);
    }
}

impl<Rend: QuadRenderer, Msg, Evt: Debug, Theme> From<Rectangle>
    for Element<Rend, Msg, Evt, Theme>
{
    fn from(value: Rectangle) -> Self {
        Element::new(value)
    }
}

impl WidgetExt for Rectangle {}
