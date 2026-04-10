mod scrollable_theme;
mod scrollbar_state;

use minlin::{Infinity, MapExt, Padding, Rect, RectExt, Vec2};

use crate::{LayoutBounds, RelPos, Widget};

pub use self::{scrollable_theme::*, scrollbar_state::*};

#[derive(Debug)]
pub struct Scrollable<W, Style> {
    pub child: W,
    pub style: Style,
    pub scrollbar: Vec2<ScrollbarState>,
    pub rel_scroll: Vec2<f32>,
    pub padding: Padding<f32>,
    abs_scroll: Vec2<f32>,
    vbounds: Rect<f32>,
    rel_pos: RelPos,
    bounds: Rect<f32>,
}

impl<W, Style> Scrollable<W, Style> {
    pub fn with_scrollbars_styled(
        style: Style,
        scrollbar: impl Into<Vec2<ScrollbarState>>,
        child: W,
    ) -> Self {
        Self {
            child,
            style,
            scrollbar: scrollbar.into(),
            rel_scroll: Vec2::ZERO,
            padding: Padding::default(),
            abs_scroll: Vec2::default(),
            vbounds: Rect::default(),
            rel_pos: RelPos::default(),
            bounds: Rect::default(),
        }
    }

    pub fn vertical_styled(style: Style, child: W) -> Self {
        Self::with_scrollbars_styled(
            style,
            [ScrollbarState::Disabled, ScrollbarState::Visible],
            child,
        )
    }

    pub fn horizontal_styled(style: Style, child: W) -> Self {
        Self::with_scrollbars_styled(
            style,
            [ScrollbarState::Visible, ScrollbarState::Disabled],
            child,
        )
    }

    pub fn both_styled(style: Style, child: W) -> Self {
        Self::with_scrollbars_styled(
            style,
            [ScrollbarState::Visible, ScrollbarState::Visible],
            child,
        )
    }

    pub fn styled(style: Style, child: W) -> Self {
        Self::vertical_styled(style, child)
    }

    pub fn scrollbar(
        &mut self,
        scrollbar: impl Into<Vec2<ScrollbarState>>,
        child: W,
    ) -> &mut Self {
        self.scrollbar = scrollbar.into();
        self
    }

    pub fn rel_scroll(
        &mut self,
        rel_scroll: impl Into<Vec2<f32>>,
    ) -> &mut Self {
        self.rel_scroll = rel_scroll.into();
        self
    }

    pub fn padding(&mut self, padding: impl Into<Padding<f32>>) -> &mut Self {
        self.padding = padding.into();
        self
    }
}

impl<W, Style: Default> Scrollable<W, Style> {
    pub fn with_scrollbars(
        scrollbar: impl Into<Vec2<ScrollbarState>>,
        child: W,
    ) -> Self {
        Self::with_scrollbars_styled(Style::default(), scrollbar, child)
    }

    pub fn vertical(child: W) -> Self {
        Self::vertical_styled(Style::default(), child)
    }

    pub fn horizontal(child: W) -> Self {
        Self::horizontal_styled(Style::default(), child)
    }

    pub fn both(child: W) -> Self {
        Self::both_styled(Style::default(), child)
    }

    pub fn new(child: W) -> Self {
        Self::styled(Style::default(), child)
    }
}

impl<W, Rend, Msg, Evt, Theme> Widget<Rend, Msg, Evt, Theme>
    for Scrollable<W, Theme::Style>
where
    W: Widget<Rend, Msg, Evt, Theme>,
    Theme: ScrollableTheme,
{
    fn layout(
        &mut self,
        lp: &mut crate::LayoutParams<'_, Rend, Msg, Theme>,
        bounds: &crate::LayoutBounds,
        pos_base: RelPos,
        flags: crate::LayoutFlags,
    ) -> Rect<f32> {
        self.rel_pos.update(pos_base.clone());
        self.bounds = bounds.best_max();

        let enabled = self.scrollbar.map(|a| a.enabled());
        let size = (self.bounds.size() - lp.theme.scrollbar_size(&self.style))
            .combine(Vec2::INFINITY, enabled);
        let best = size.combine(Vec2::ZERO, enabled);

        self.vbounds = Rect::from_pos_size(self.bounds.pos(), size)
            .pad_rect(self.padding);

        let mut cspace = LayoutBounds::at_most(Rect::from_pos_size(
            self.vbounds.pos(),
            size,
        ));
        cspace.size.best = Some(best);
        let cbounds = self.child.layout(lp, &cspace, pos_base, flags);

        let cvx = cbounds.width() - self.vbounds.width();
        let xover = self.scrollbar.x.enabled() && cvx > 0.;
        let cvy = cbounds.height() - self.vbounds.height();
        let yover = self.scrollbar.y.enabled() && cvy > 0.;

        self.abs_scroll.x = if xover { cvx * self.rel_scroll.x } else { 0. };
        self.abs_scroll.y = if yover { cvy * self.rel_scroll.y } else { 0. };

        if self.abs_scroll != Vec2::ZERO {
            self.child
                .reposition(lp.theme, self.vbounds.pos() - self.abs_scroll);
        }

        self.bounds
    }

    fn size(&mut self, theme: &Theme) -> Vec2<f32> {
        self.child.size(theme)
            + theme.scrollbar_size(&self.style)
            + self.padding.size()
    }

    fn reposition(&mut self, theme: &Theme, pos: Vec2<f32>) {
        self.bounds.set_pos(pos);
        self.vbounds.set_pos(pos + self.padding.offset());
        self.child
            .reposition(theme, self.vbounds.pos() - self.abs_scroll);
    }

    fn event(
        &mut self,
        shell: &mut crate::Shell<Msg>,
        theme: &Theme,
        event: &crate::event::EventInfo<Evt>,
    ) -> bool {
        todo!()
    }

    fn draw(
        &mut self,
        shell: &mut crate::Shell<Msg>,
        theme: &Theme,
        renderer: &mut Rend,
    ) {
        todo!()
    }
}
