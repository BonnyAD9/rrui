use std::fmt::Debug;

use minlin::{Rect, RectExt, Vec2};

use crate::{
    Background, Border, Element, LayoutBounds, LayoutParams, QuadRenderer,
    RedrawSlot, RelPos, RelayoutSlot, Shell, Widget, WidgetExt,
    event::EventInfo,
};

#[derive(Debug)]
pub struct Rectangle {
    pub background: RedrawSlot<Background>,
    pub border: RedrawSlot<Border>,
    pub size: RelayoutSlot<Vec2<f32>>,
    bounds: Rect<f32>,
    rel_pos: RelPos,
}

impl Rectangle {
    pub fn new(
        size: impl Into<Vec2<f32>>,
        bg: impl Into<Background>,
        border: impl Into<Border>,
    ) -> Self {
        Self {
            background: bg.into().into(),
            border: border.into().into(),
            size: size.into().into(),
            bounds: Rect::default(),
            rel_pos: RelPos::new(),
        }
    }
}

impl<Rend: QuadRenderer, Msg, Evt: Debug, Theme> Widget<Rend, Msg, Evt, Theme>
    for Rectangle
{
    fn layout(
        &mut self,
        _: &mut LayoutParams<Rend, Msg, Theme>,
        bounds: &LayoutBounds,
        rel_pos: RelPos,
    ) -> Rect<f32> {
        self.size.update();

        self.rel_pos.update(rel_pos);
        self.bounds = bounds.clamp(*self.size);
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
        self.background.update();
        self.border.update();

        renderer.draw_border(self.bounds, *self.border, &*self.background);
    }

    fn size(&mut self, _: &Theme) -> Vec2<f32> {
        self.size.update();
        *self.size
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
