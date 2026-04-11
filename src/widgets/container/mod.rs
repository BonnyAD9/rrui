mod container_appereance;
mod container_theme;

pub use self::{container_appereance::*, container_theme::*};

use std::fmt::Debug;

use minlin::{Infinity, MapExt, Padding, Rect, RectExt, Vec2};

use crate::{
    Element, LayoutBounds, LayoutFlags, LayoutParams, QuadRenderer, RelPos,
    RelayoutSlot, Shell, Size, Widget, WidgetExt,
    event::{Event, EventInfo},
};

#[derive(Debug, Default)]
pub struct Container<W, S> {
    pub child: W,
    pub style: RelayoutSlot<S>,
    pub padding: Padding<Size>,
    pub size: Option<Vec2<f32>>,
    child_offset: Vec2<f32>,
    bounds: Rect<f32>,
    rel_pos: RelPos,
}

impl<W, S> Container<W, S> {
    pub fn styled(style: S, child: W) -> Self {
        Self {
            child,
            style: style.into(),
            padding: Padding::default(),
            child_offset: Vec2::default(),
            size: None,
            bounds: Rect::default(),
            rel_pos: RelPos::default(),
        }
    }

    pub fn center_styled(style: S, child: W) -> Self {
        Self {
            child,
            style: style.into(),
            padding: Size::Relative(1.).into(),
            child_offset: Vec2::default(),
            size: None,
            bounds: Rect::default(),
            rel_pos: RelPos::default(),
        }
    }

    pub fn padding(&mut self, total: impl Into<Padding<Size>>) -> &mut Self {
        self.padding = total.into();
        self
    }

    pub fn pad_abs(&mut self, absolute: impl Into<Padding<f32>>) -> &mut Self {
        self.padding = absolute.into().map(Size::Absolute);
        self
    }

    pub fn pad_rel(&mut self, relative: impl Into<Padding<f32>>) -> &mut Self {
        self.padding = relative.into().map(Size::Relative);
        self
    }

    pub fn size(&mut self, size: impl Into<Vec2<f32>>) -> &mut Self {
        self.size = size.into().into();
        self
    }
}

impl<W, S: Default> Container<W, S> {
    pub fn new(child: W) -> Self {
        Self::styled(S::default(), child)
    }

    pub fn center(child: W) -> Self {
        Self::center_styled(S::default(), child)
    }
}

impl<W, Rend, Msg, Evt, Theme, Style> Widget<Rend, Msg, Evt, Theme>
    for Container<W, Style>
where
    W: Widget<Rend, Msg, Evt, Theme>,
    Rend: QuadRenderer,
    Evt: Event,
    Theme: ContainerTheme<Style = Style>,
{
    fn layout(
        &mut self,
        lp: &mut LayoutParams<'_, Rend, Msg, Evt, Theme>,
        bounds: &LayoutBounds,
        rel_pos: RelPos,
        flags: LayoutFlags,
    ) -> Rect<f32> {
        self.rel_pos.update(rel_pos.clone());

        let bw = lp.theme.border_width(&self.style);
        let abs_pad = self.padding.map(|a| a.to_parts().x + bw);

        let bounds = if let Some(s) = self.size {
            LayoutBounds::filling(bounds.clamp(s))
        } else {
            *bounds
        };

        let cbounds = bounds.padded(abs_pad);
        let cbounds =
            self.child.layout(lp, &cbounds, rel_pos, flags) - abs_pad;

        let remaining = bounds.size.best_max() - cbounds.size();
        let rel_pad = self.padding.map(|a| a.to_parts().y);
        let pad = Self::resolve_padding(remaining, rel_pad);

        let offset = pad.offset();
        self.child_offset = offset + abs_pad.offset();

        if offset.x != 0. || offset.y != 0. {
            self.child
                .reposition(lp.theme, self.child_offset + bounds.pos);
        }

        self.bounds = if self.size.is_some() {
            bounds.best_max()
        } else {
            Rect::from_pos_size(cbounds.pos(), cbounds.size() + pad.size())
        };
        self.bounds
    }

    fn size(&mut self, theme: &Theme) -> Vec2<f32> {
        if let Some(s) = self.size {
            return s;
        }

        let size = self.padding.size();
        let rel = size.map(|a| a.y != 0.);

        if rel.x && rel.y {
            return Vec2::INFINITY;
        }

        let mut cs = self.child.size(theme);
        let bw = theme.border_width(&self.style) * 2.;
        cs.x += size.x.x + bw;
        cs.y += size.y.x + bw;

        cs.combine(Vec2::INFINITY, rel)
    }

    fn reposition(&mut self, theme: &Theme, pos: Vec2<f32>) {
        self.bounds.set_pos(pos);
        self.child.reposition(theme, pos + self.child_offset);
    }

    fn event(
        &mut self,
        shell: &mut Shell<Rend, Msg, Evt, Theme>,
        theme: &Theme,
        event: &EventInfo<Evt>,
    ) -> bool {
        let bw = theme.border_width(&self.style);
        let bounds = self.rel_pos.position_rect(self.bounds);
        let cb = if bw == 0. {
            bounds
        } else {
            bounds.pad_rect(bw)
        };
        if event.is_for(cb) {
            self.child.event(shell, theme, event)
        } else {
            false
        }
    }

    fn draw(
        &mut self,
        shell: &mut Shell<Rend, Msg, Evt, Theme>,
        theme: &Theme,
        renderer: &mut Rend,
    ) {
        if let Some(a) = theme.appereance(&self.style) {
            let bounds = self.rel_pos.position_rect(self.bounds);
            renderer.draw_border(bounds, a.border, a.background);
        }
        self.child.draw(shell, theme, renderer);
    }
}

impl<W, Style> Container<W, Style> {
    fn resolve_padding1d(
        available: f32,
        rstart: f32,
        rend: f32,
    ) -> (f32, f32) {
        let rel = rstart + rend;
        if rel == 0. {
            return (0., 0.);
        }
        let unit = available / rel;
        (rstart * unit, rend * unit)
    }

    fn resolve_padding(
        available: Vec2<f32>,
        rel_pad: Padding<f32>,
    ) -> Padding<f32> {
        let (l, r) = Self::resolve_padding1d(
            available.x,
            rel_pad.left(),
            rel_pad.right(),
        );
        let (t, b) = Self::resolve_padding1d(
            available.y,
            rel_pad.top(),
            rel_pad.bottom(),
        );
        Padding::new(l, t, r, b)
    }
}

impl<W, Rend, Msg, Evt, Theme, Style> From<Container<W, Style>>
    for Element<Rend, Msg, Evt, Theme>
where
    W: Widget<Rend, Msg, Evt, Theme> + 'static,
    Rend: QuadRenderer,
    Evt: Event,
    Theme: ContainerTheme<Style = Style>,
    Style: 'static,
{
    fn from(value: Container<W, Style>) -> Self {
        Element::new(value)
    }
}

impl<W, S> WidgetExt for Container<W, S> {}
