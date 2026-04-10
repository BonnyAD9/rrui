mod part_scrollbar;
mod scrollbar_event;
mod scrollbar_sizes;
mod scrollbar_state;
mod scrollbar_style;
mod scrollbar_theme;

use crate::{
    Element, Orientation, QuadRenderer, RedrawSlot, RelPos, SvgRenderer,
    Widget, WidgetExt, event::Event,
};

pub use self::{
    part_scrollbar::*, scrollbar_event::*, scrollbar_sizes::*,
    scrollbar_state::*, scrollbar_style::*, scrollbar_theme::*,
};

pub struct Scrollbar<Style: ScrollbarStyle, Msg> {
    pub on_scroll: Box<dyn FnMut(f32) -> Option<Msg>>,
    pub inner: PartScrollbar<Style>,
    rel_pos: RelPos,
}

impl<Style: ScrollbarStyle, Msg> Scrollbar<Style, Msg> {
    pub fn styled(style: Style, orientation: Orientation) -> Self {
        Self {
            on_scroll: Box::new(|_| None),
            inner: PartScrollbar::styled(style, orientation),
            rel_pos: RelPos::default(),
        }
    }

    pub fn styled_horizontal(style: Style) -> Self {
        Self::styled(style, Orientation::Horizontal)
    }

    pub fn styled_vertical(style: Style) -> Self {
        Self::styled(style, Orientation::Vertical)
    }

    pub fn new(orientation: Orientation) -> Self
    where
        Style: Default,
    {
        Self::styled(Style::default(), orientation)
    }

    pub fn horizontal() -> Self
    where
        Style: Default,
    {
        Self::new(Orientation::Horizontal)
    }

    pub fn vertical() -> Self
    where
        Style: Default,
    {
        Self::new(Orientation::Vertical)
    }

    pub fn on_scroll(
        &mut self,
        handler: impl FnMut(f32) -> Option<Msg> + 'static,
    ) -> &mut Self {
        self.on_scroll = Box::new(handler);
        self
    }

    pub fn configure(
        &mut self,
        state: impl Into<RedrawSlot<ScrollbarState>>,
    ) -> &mut Self {
        self.inner.state = state.into();
        self
    }
}

impl<Rend, Msg, Evt, Theme> Widget<Rend, Msg, Evt, Theme>
    for Scrollbar<<Theme as ScrollbarTheme>::Style, Msg>
where
    Theme: ScrollbarTheme,
    Rend: QuadRenderer + SvgRenderer,
    Evt: Event,
{
    fn layout(
        &mut self,
        lp: &mut crate::LayoutParams<'_, Rend, Msg, Theme>,
        bounds: &crate::LayoutBounds,
        pos_base: RelPos,
        _: crate::LayoutFlags,
    ) -> minlin::Rect<f32> {
        self.rel_pos.update(pos_base);
        self.inner.layout(lp.theme, lp.shell, bounds)
    }

    fn size(&mut self, theme: &Theme) -> minlin::Vec2<f32> {
        self.inner.size(theme)
    }

    fn reposition(&mut self, _: &Theme, pos: minlin::Vec2<f32>) {
        self.inner.reposition(pos);
    }

    fn event(
        &mut self,
        shell: &mut crate::Shell<Msg>,
        theme: &Theme,
        event: &crate::event::EventInfo<Evt>,
    ) -> bool {
        let (handled, evt) =
            self.inner.event(self.rel_pos.get(), shell, theme, event);
        if let ScrollbarEvent::ScrollTo(pos) = evt {
            shell.msgs((self.on_scroll)(pos));
        }
        handled
    }

    fn draw(
        &mut self,
        _: &mut crate::Shell<Msg>,
        theme: &Theme,
        renderer: &mut Rend,
    ) {
        self.inner.draw(self.rel_pos.get(), theme, renderer);
    }
}

impl<Rend, Msg, Evt, Theme>
    From<Scrollbar<<Theme as ScrollbarTheme>::Style, Msg>>
    for Element<Rend, Msg, Evt, Theme>
where
    Theme: ScrollbarTheme,
    <Theme as ScrollbarTheme>::Style: 'static,
    Msg: 'static,
    Rend: QuadRenderer + SvgRenderer,
    Evt: Event,
{
    fn from(value: Scrollbar<<Theme as ScrollbarTheme>::Style, Msg>) -> Self {
        Self::new(value)
    }
}

impl<Style, Msg> WidgetExt for Scrollbar<Style, Msg> where Style: ScrollbarStyle
{}
