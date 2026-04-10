mod button_event;
mod button_state;
mod button_theme;
mod part_button;

use std::{borrow::Cow, fmt::Debug};

use crate::{
    Element, LayoutFlags, LayoutParams, QuadRenderer, RedrawSlot, RelPos,
    TextAlign, Widget, WidgetExt,
    event::{Event, MouseButton, MouseState},
    widgets::TextBlock,
};

pub use self::{
    button_event::*, button_state::*, button_theme::*, part_button::*,
};

use minlin::{Padding, Rect, Vec2};

pub struct Button<W, Style, Msg> {
    pub child: W,
    pub on_press: Box<dyn FnMut(MouseButton) -> Option<Msg>>,
    pub inner: PartButton<Style>,
    rel_pos: RelPos,
}

impl<W, Style, Msg> Button<W, Style, Msg> {
    pub fn styled(style: Style, child: W) -> Self {
        Self {
            child,
            on_press: Box::new(|_| None),
            inner: PartButton::styled(style.into()),
            rel_pos: RelPos::default(),
        }
    }

    pub fn new(child: W) -> Self
    where
        Style: Default,
    {
        Self::styled(Style::default(), child)
    }

    pub fn padding(&mut self, total: impl Into<Padding<f32>>) -> &mut Self {
        self.inner.padding = total.into();
        self
    }

    pub fn style(&mut self, s: impl Into<RedrawSlot<Style>>) -> &mut Self {
        self.inner.style = s.into();
        self
    }

    pub fn on_press(
        &mut self,
        handler: impl FnMut(MouseButton) -> Option<Msg> + 'static,
    ) -> &mut Self {
        self.on_press = Box::new(handler);
        self
    }

    pub fn size(&mut self, size: impl Into<Vec2<f32>>) -> &mut Self {
        self.inner.size = Some(size.into());
        self
    }

    pub fn react(&mut self, react: MouseState) -> &mut Self {
        self.inner.react = react;
        self
    }
}

impl<BStyle: Default, Msg, TStyle: Default, Font, LText>
    Button<TextBlock<TStyle, Font, LText>, BStyle, Msg>
{
    pub fn text(text: impl Into<Cow<'static, str>>) -> Self {
        let mut res = Self::new(TextBlock::new(text));
        res.child.align_x = TextAlign::Center;
        res
    }
}

impl<W: Default, Style: Default, Msg> Default for Button<W, Style, Msg> {
    fn default() -> Self {
        Self::new(W::default())
    }
}

impl<W: Debug, Style: Debug, Msg> Debug for Button<W, Style, Msg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Button")
            .field("child", &self.child)
            .field("on_press", &"closure")
            .field("inner", &self.inner)
            .field("rel_pos", &self.rel_pos)
            .finish()
    }
}

impl<W, Rend, Msg, Evt, Theme, Style> Widget<Rend, Msg, Evt, Theme>
    for Button<W, Style, Msg>
where
    W: Widget<Rend, Msg, Evt, Theme>,
    Rend: QuadRenderer,
    Evt: Event,
    Theme: ButtonTheme<Style = Style>,
{
    fn layout(
        &mut self,
        lp: &mut LayoutParams<'_, Rend, Msg, Theme>,
        bounds: &crate::LayoutBounds,
        rel_pos: RelPos,
        flags: LayoutFlags,
    ) -> Rect<f32> {
        self.rel_pos.update(rel_pos.clone());
        self.inner
            .layout(bounds, |b| self.child.layout(lp, b, rel_pos, flags))
    }

    fn size(&mut self, theme: &Theme) -> Vec2<f32> {
        self.inner.size(|| self.child.size(theme))
    }

    fn reposition(&mut self, theme: &Theme, pos: Vec2<f32>) {
        self.inner
            .reposition(pos, |pos| self.child.reposition(theme, pos))
    }

    fn event(
        &mut self,
        shell: &mut crate::Shell<Msg>,
        theme: &Theme,
        event: &crate::event::EventInfo<Evt>,
    ) -> bool {
        let (handled, evt) =
            self.inner
                .event(self.rel_pos.get(), shell, theme, event, |s| {
                    self.child.event(s, theme, event)
                });
        match evt {
            ButtonEvent::Nothing => {}
            ButtonEvent::Clicked(b) => shell.msgs((self.on_press)(b)),
        }
        handled
    }

    fn draw(
        &mut self,
        shell: &mut crate::Shell<Msg>,
        theme: &Theme,
        renderer: &mut Rend,
    ) {
        self.inner.draw(self.rel_pos.get(), theme, renderer, |r| {
            self.child.draw(shell, theme, r)
        });
    }
}

impl<W, Rend, Msg, Evt, Theme, Style> From<Button<W, Style, Msg>>
    for Element<Rend, Msg, Evt, Theme>
where
    W: Widget<Rend, Msg, Evt, Theme> + 'static,
    Msg: 'static,
    Rend: QuadRenderer,
    Evt: Event,
    Theme: ButtonTheme<Style = Style>,
    Style: 'static,
{
    fn from(value: Button<W, Style, Msg>) -> Self {
        Self::new(value)
    }
}

impl<W, Style, Msg> WidgetExt for Button<W, Style, Msg> {}
