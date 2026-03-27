mod container_appereance;
mod container_theme;

pub use self::{container_appereance::*, container_theme::*};

use std::fmt::Debug;

use minlin::{MapExt, Rect, RectExt, Vec2};

use crate::{
    Element, LayoutBounds, QuadRenderer, Shell, Widget, WidgetExt,
    event::{Event, EventInfo},
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Container<W, S> {
    pub child: W,
    pub style: S,
    bounds: Rect<f32>,
}

impl<W, S> Container<W, S> {
    pub fn styled(style: S, child: W) -> Self {
        Self {
            child,
            style,
            bounds: Rect::default(),
        }
    }

    pub fn style(&mut self, style: S) -> &mut Self {
        self.style = style;
        self
    }
}

impl<W, S: Default> Container<W, S> {
    pub fn new(child: W) -> Self {
        Self::styled(S::default(), child)
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
        shell: &mut Shell,
        theme: &Theme,
        bounds: &LayoutBounds,
        renderer: &Rend,
    ) -> Rect<f32> {
        let bw = theme.border_width(&self.style);
        if bw != 0. {
            let bounds = bounds.padded(bw);
            self.bounds = self
                .child
                .layout(shell, theme, &bounds, renderer)
                .extend_rect(bw);
            self.bounds
        } else {
            self.bounds = self.child.layout(shell, theme, bounds, renderer);
            self.bounds
        }
    }

    fn size(&self, theme: &Theme) -> Vec2<f32> {
        let bw = theme.border_width(&self.style) * 2.;
        self.child.size(theme).map(|a| a + bw)
    }

    fn reposition(&mut self, theme: &Theme, pos: Vec2<f32>) {
        let bw = theme.border_width(&self.style);
        self.child.reposition(theme, pos.map(|a| a + bw));
    }

    fn event(
        &mut self,
        shell: &mut Shell,
        theme: &Theme,
        event: &EventInfo<Evt>,
    ) -> bool {
        let bw = theme.border_width(&self.style);
        let cb = if bw == 0. {
            self.bounds
        } else {
            self.bounds.pad_rect(bw)
        };
        if event.is_for(cb) {
            self.child.event(shell, theme, event)
        } else {
            false
        }
    }

    fn draw(&mut self, shell: &mut Shell, theme: &Theme, renderer: &mut Rend) {
        if let Some(a) = theme.appereance(&self.style) {
            renderer.draw_border(self.bounds, a.border, a.background);
        }
        self.child.draw(shell, theme, renderer);
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
