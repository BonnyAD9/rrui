mod debug_ext;

pub use self::debug_ext::*;

use std::ops::{Deref, DerefMut};

use minlin::{Rect, RectExt};

use crate::{
    Border, Color, Element, Quad, QuadRenderer, RelPos, Widget, WidgetExt,
};

#[derive(Debug)]
pub struct Debug<W> {
    pub child: W,
    pub draw_after: bool,
    pub border: Color,
    pub background: Color,
    pub thickness: f32,
    pub enabled: bool,
    rel_pos: RelPos,
    bounds: Rect<f32>,
}

impl<W> Debug<W> {
    pub fn new(
        draw_after: bool,
        border: impl Into<Color>,
        background: impl Into<Color>,
        thickness: f32,
        child: W,
    ) -> Self {
        Self {
            child,
            draw_after,
            border: border.into(),
            background: background.into(),
            thickness,
            enabled: true,
            rel_pos: RelPos::default(),
            bounds: Rect::default(),
        }
    }

    pub fn color(color: impl Into<Color>, child: W) -> Self {
        let c = color.into();
        let mut bg = c;
        bg.w *= 0.05;
        Self::new(false, c, bg, 1., child)
    }

    pub fn r(child: W) -> Self {
        Self::color(Color::xrgb(0xff0000), child)
    }

    pub fn g(child: W) -> Self {
        Self::color(Color::xrgb(0x00ff00), child)
    }

    pub fn b(child: W) -> Self {
        Self::color(Color::xrgb(0x0000ff), child)
    }

    pub fn c(child: W) -> Self {
        Self::color(Color::xrgb(0x00ffff), child)
    }

    pub fn m(child: W) -> Self {
        Self::color(Color::xrgb(0xff00ff), child)
    }

    pub fn y(child: W) -> Self {
        Self::color(Color::xrgb(0xffff00), child)
    }

    pub fn black(child: W) -> Self {
        Self::color(Color::BLACK, child)
    }

    pub fn white(child: W) -> Self {
        Self::color(Color::WHITE, child)
    }

    pub fn off(mut self) -> Self {
        self.enabled = false;
        self
    }

    pub fn on(mut self) -> Self {
        self.enabled = true;
        self
    }
}

impl<W, Rend, Msg, Evt, Theme> Widget<Rend, Msg, Evt, Theme> for Debug<W>
where
    Rend: QuadRenderer,
    W: Widget<Rend, Msg, Evt, Theme>,
{
    fn layout(
        &mut self,
        lp: &mut crate::LayoutParams<'_, Rend, Msg, Theme>,
        bounds: &crate::LayoutBounds,
        pos_base: crate::RelPos,
        flags: crate::LayoutFlags,
    ) -> Rect<f32> {
        self.rel_pos.update(pos_base.clone());
        self.bounds = self.child.layout(lp, bounds, pos_base, flags);
        self.bounds
    }

    fn size(&mut self, theme: &Theme) -> minlin::Vec2<f32> {
        self.child.size(theme)
    }

    fn reposition(&mut self, theme: &Theme, pos: minlin::Vec2<f32>) {
        self.bounds.set_pos(pos);
        self.child.reposition(theme, pos);
    }

    fn event(
        &mut self,
        shell: &mut crate::Shell<Msg>,
        theme: &Theme,
        event: &crate::event::EventInfo<Evt>,
    ) -> bool {
        self.child.event(shell, theme, event)
    }

    fn draw(
        &mut self,
        shell: &mut crate::Shell<Msg>,
        theme: &Theme,
        renderer: &mut Rend,
    ) {
        if !self.enabled {
            self.child.draw(shell, theme, renderer);
            return;
        }

        let bounds = self.rel_pos.position_rect(self.bounds);

        if self.draw_after {
            self.child.draw(shell, theme, renderer);
            renderer.draw_quad(
                &Quad::border(
                    bounds,
                    Border::square(self.border, self.thickness),
                ),
                self.background,
            );
        } else {
            renderer.draw_quad(
                &Quad::border(
                    bounds,
                    Border::square(self.border, self.thickness),
                ),
                self.background,
            );
            self.child.draw(shell, theme, renderer);
        }
    }
}

impl<W, Rend, Msg, Evt, Theme> From<Debug<W>>
    for Element<Rend, Msg, Evt, Theme>
where
    Rend: QuadRenderer,
    W: Widget<Rend, Msg, Evt, Theme> + 'static,
{
    fn from(value: Debug<W>) -> Self {
        Self::new(value)
    }
}

impl<W> WidgetExt for Debug<W> {}

impl<W> Deref for Debug<W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<W> DerefMut for Debug<W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.child
    }
}
