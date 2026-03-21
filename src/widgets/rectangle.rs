use std::fmt::Debug;

use minlin::{Rect, Vec2};

use crate::{
    Background, Border, Element, LayoutBounds, QuadRenderer, Shell, Widget,
};

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

impl<Rend: QuadRenderer, Msg, Evt: Debug> Widget<Rend, Msg, Evt>
    for Rectangle
{
    fn layout(&mut self, _: &mut Shell, bounds: &LayoutBounds) -> Rect<f32> {
        self.bounds = bounds.clamp(self.size);
        self.bounds
    }

    fn event(&mut self, _: &mut Shell, _: &Evt) {}

    fn draw(&mut self, _: &mut Shell, renderer: &mut Rend) {
        renderer.draw_border(self.bounds, self.border, &self.background);
    }
}

impl<Rend: QuadRenderer, Msg, Evt: Debug> From<Rectangle>
    for Element<Rend, Msg, Evt>
{
    fn from(value: Rectangle) -> Self {
        Element::new(value)
    }
}
