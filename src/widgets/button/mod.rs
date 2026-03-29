mod button_state;
mod button_theme;

use std::{borrow::Cow, fmt::Debug};

use crate::{
    Element, LayoutBounds, LayoutParams, QuadRenderer, RedrawSlot, RelPos,
    TextAlign, Widget, WidgetExt,
    event::{Event, EventKind, MouseButton, MouseRelation, MouseState},
    widgets::TextBlock,
};

pub use self::{button_state::*, button_theme::*};

use minlin::{Padding, Rect, RectExt, Vec2};

pub struct Button<W, Style, Msg> {
    pub child: W,
    pub style: RedrawSlot<Style>,
    pub size: Option<Vec2<f32>>,
    pub padding: Padding<f32>,
    pub on_press: Box<dyn FnMut(MouseButton) -> Option<Msg>>,
    pub react: MouseState,
    bounds: Rect<f32>,
    state: ButtonState,
    rel_pos: RelPos,
}

impl<W, Style, Msg> Button<W, Style, Msg> {
    pub fn styled(style: Style, child: W) -> Self {
        Self {
            child,
            style: style.into(),
            size: None,
            padding: Padding::uniform(5.),
            bounds: Rect::default(),
            state: ButtonState::Normal,
            react: MouseState::LEFT,
            on_press: Box::new(|_| None),
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
        self.padding = total.into();
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
        self.size = Some(size.into());
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
            .field("style", &self.style)
            .field("size", &self.size)
            .field("padding", &self.padding)
            .field("on_press", &"Box<FnMut>")
            .field("bounds", &self.bounds)
            .field("state", &self.state)
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
    ) -> Rect<f32> {
        self.rel_pos.update(rel_pos.clone());

        if let Some(s) = self.size {
            self.bounds = bounds.clamp(s);
            let mut bounds =
                LayoutBounds::at_most(self.bounds.pad_rect(self.padding));
            bounds.fill();
            self.child.layout(lp, &bounds, rel_pos);
            self.bounds
        } else {
            let bounds = bounds.padded(self.padding);
            self.bounds = self
                .child
                .layout(lp, &bounds, rel_pos)
                .extend_rect(self.padding);
            self.bounds
        }
    }

    fn size(&mut self, theme: &Theme) -> Vec2<f32> {
        if let Some(s) = self.size {
            s
        } else {
            self.child.size(theme) + self.padding.size()
        }
    }

    fn reposition(&mut self, theme: &Theme, pos: Vec2<f32>) {
        self.bounds.set_pos(pos);
        self.child.reposition(theme, pos + self.padding.offset());
    }

    fn event(
        &mut self,
        shell: &mut crate::Shell<Msg>,
        theme: &Theme,
        event: &crate::event::EventInfo<Evt>,
    ) -> bool {
        let bounds = self.rel_pos.position_rect(self.bounds);

        let new_state = match event.mouse_relate_to(bounds) {
            MouseRelation::None | MouseRelation::Elswhere => return false,
            MouseRelation::Hover
            | MouseRelation::Move
            | MouseRelation::Enter => {
                if shell.mouse_state().intersects(self.react) {
                    ButtonState::Pressed
                } else {
                    ButtonState::Hover
                }
            }
            MouseRelation::Leave => ButtonState::Normal,
        };

        let handled = match event.get_kind() {
            EventKind::MousePress(b) if self.react.contains(b.into()) => {
                shell.msgs((self.on_press)(b));
                true
            }
            EventKind::MouseRelease(b) if self.react.contains(b.into()) => {
                true
            }
            _ => false,
        };

        if self.state != new_state
            && theme.is_different(&self.style, self.state, new_state)
        {
            shell.request_redraw();
        }

        self.state = new_state;
        handled
    }

    fn draw(
        &mut self,
        shell: &mut crate::Shell<Msg>,
        theme: &Theme,
        renderer: &mut Rend,
    ) {
        self.style.update();

        if let Some(a) = theme.appereance(&self.style, self.state) {
            let bounds = self.rel_pos.position_rect(self.bounds);
            renderer.draw_border(bounds, a.border, a.background);
        }
        self.child.draw(shell, theme, renderer);
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
