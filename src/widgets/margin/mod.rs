mod margin_ext;
use std::ops::{Deref, DerefMut};

pub use self::margin_ext::*;

use minlin::{Padding, Rect, Vec2};

use crate::{
    Element, LayoutBounds, LayoutFlags, LayoutParams, RelPos, Widget,
    WidgetExt, event::EventInfo,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Margin<W> {
    pub margin: Padding<f32>,
    pub child: W,
}

impl<W> Margin<W> {
    pub fn new(margin: impl Into<Padding<f32>>, child: W) -> Self {
        Self {
            margin: margin.into(),
            child,
        }
    }

    pub fn marge(&mut self, additional: impl Into<Padding<f32>>) -> &mut Self {
        self.margin += additional.into();
        self
    }

    pub fn margin(&mut self, total: impl Into<Padding<f32>>) -> &mut Self {
        self.margin = total.into();
        self
    }

    pub fn set_margin(&mut self, total: impl Into<Padding<f32>>) -> &mut Self {
        self.margin(total)
    }
}

impl<W> Deref for Margin<W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<W> DerefMut for Margin<W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.child
    }
}

impl<W, Rend, Msg, Evt, Theme> Widget<Rend, Msg, Evt, Theme> for Margin<W>
where
    W: Widget<Rend, Msg, Evt, Theme>,
{
    fn layout(
        &mut self,
        lp: &mut LayoutParams<'_, Rend, Msg, Evt, Theme>,
        bounds: &LayoutBounds,
        rel_pos: RelPos,
        flags: LayoutFlags,
    ) -> Rect<f32> {
        let cbounds = bounds.padded(self.margin);
        let cbounds = self.child.layout(lp, &cbounds, rel_pos, flags);
        bounds.extend_rect_within(cbounds, self.margin)
    }

    fn size(&mut self, theme: &Theme) -> Vec2<f32> {
        self.child.size(theme) + self.margin.size()
    }

    fn reposition(&mut self, theme: &Theme, pos: Vec2<f32>) {
        self.child.reposition(theme, pos + self.margin.offset());
    }

    fn event(
        &mut self,
        shell: &mut crate::Shell<Rend, Msg, Evt, Theme>,
        theme: &Theme,
        event: &EventInfo<Evt>,
    ) -> bool {
        self.child.event(shell, theme, event)
    }

    fn draw(
        &mut self,
        shell: &mut crate::Shell<Rend, Msg, Evt, Theme>,
        theme: &Theme,
        renderer: &mut Rend,
    ) {
        self.child.draw(shell, theme, renderer)
    }
}

impl<W, Rend, Msg, Evt, Theme> From<Margin<W>>
    for Element<Rend, Msg, Evt, Theme>
where
    W: Widget<Rend, Msg, Evt, Theme> + 'static,
{
    fn from(value: Margin<W>) -> Self {
        Element::new(value)
    }
}

impl<W> WidgetExt for Margin<W> {}
