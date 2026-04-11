use minlin::{Padding, Rect, RectExt, Vec2};

use crate::{
    LayoutBounds, QuadRenderer, RedrawSlot, Shell,
    event::{Event, EventInfo, EventKind, MouseRelation, MouseState},
    widgets::{ButtonEvent, ButtonState, ButtonTheme},
};

#[derive(Debug)]
pub struct PartButton<Style> {
    pub style: RedrawSlot<Style>,
    pub size: Option<Vec2<f32>>,
    pub padding: Padding<f32>,
    pub react: MouseState,
    bounds: Rect<f32>,
    state: ButtonState,
}

impl<Style> PartButton<Style> {
    pub fn styled(style: RedrawSlot<Style>) -> Self {
        Self {
            style,
            size: None,
            padding: Padding::default(),
            react: MouseState::LEFT,
            bounds: Rect::default(),
            state: ButtonState::Normal,
        }
    }

    pub fn bounds(&self) -> Rect<f32> {
        self.bounds
    }

    pub fn off_bounds(&self, off: Vec2<f32>) -> Rect<f32> {
        Rect::from_pos_size(self.bounds.pos() + off, self.bounds.size())
    }

    pub fn disable<Rend, Msg, Evt, Theme>(
        &mut self,
        theme: &Theme,
        shell: &mut Shell<Rend, Msg, Evt, Theme>,
    ) where
        Theme: ButtonTheme<Style = Style>,
    {
        if self.state.is_disabled() {
            return;
        }
        if theme.is_different(&self.style, self.state, ButtonState::Disabled) {
            shell.request_redraw();
        }
        self.state = ButtonState::Disabled;
    }

    pub fn enable<Rend, Msg, Evt, Theme>(
        &mut self,
        theme: &Theme,
        shell: &mut Shell<Rend, Msg, Evt, Theme>,
    ) where
        Theme: ButtonTheme<Style = Style>,
    {
        if !self.state.is_disabled() {
            return;
        }
        if theme.is_different(&self.style, self.state, ButtonState::Normal) {
            shell.request_redraw();
        }
        self.state = ButtonState::Normal;
    }

    pub fn set_disable<Rend, Msg, Evt, Theme>(
        &mut self,
        val: bool,
        theme: &Theme,
        shell: &mut Shell<Rend, Msg, Evt, Theme>,
    ) where
        Theme: ButtonTheme<Style = Style>,
    {
        if val {
            self.disable(theme, shell);
        } else {
            self.enable(theme, shell);
        }
    }

    pub fn set_disable_no_redraw(&mut self, val: bool) {
        if val {
            self.state = ButtonState::Disabled;
        } else if self.state.is_disabled() {
            self.state = ButtonState::Normal;
        }
    }

    pub fn state(&self) -> ButtonState {
        self.state
    }

    pub fn layout(
        &mut self,
        bounds: &LayoutBounds,
        layout_child: impl FnOnce(&LayoutBounds) -> Rect<f32>,
    ) -> Rect<f32> {
        if let Some(s) = self.size {
            self.bounds = bounds.clamp(s);
            let mut bounds =
                LayoutBounds::at_most(self.bounds.pad_rect(self.padding));
            bounds.fill();
            layout_child(&bounds);
        } else {
            let bounds = bounds.padded(self.padding);
            self.bounds = layout_child(&bounds).extend_rect(self.padding);
        }
        self.bounds
    }

    pub fn layout_direct(&mut self, bounds: Rect<f32>) {
        self.bounds = bounds;
    }

    pub fn size(&self, child_size: impl FnOnce() -> Vec2<f32>) -> Vec2<f32> {
        if let Some(s) = self.size {
            s
        } else {
            child_size() + self.padding.size()
        }
    }

    pub fn reposition(
        &mut self,
        pos: Vec2<f32>,
        reposition_child: impl FnOnce(Vec2<f32>),
    ) {
        self.bounds.set_pos(pos);
        reposition_child(pos + self.padding.offset());
    }

    pub fn reposition_direct(&mut self, pos: Vec2<f32>) {
        self.bounds.set_pos(pos);
    }

    pub fn event<Rend, Msg, Evt, Theme>(
        &mut self,
        off: Vec2<f32>,
        shell: &mut Shell<Rend, Msg, Evt, Theme>,
        theme: &Theme,
        event: &EventInfo<Evt>,
        child_event: impl FnOnce(&mut Shell<Rend, Msg, Evt, Theme>) -> bool,
    ) -> (bool, ButtonEvent)
    where
        Evt: Event,
        Theme: ButtonTheme<Style = Style>,
    {
        let mut res = self.event_direct(off, shell, theme, event);

        if !res.0 {
            res.0 = child_event(shell);
        }
        res
    }

    pub fn event_direct<Rend, Msg, Evt, Theme>(
        &mut self,
        off: Vec2<f32>,
        shell: &mut Shell<Rend, Msg, Evt, Theme>,
        theme: &Theme,
        event: &EventInfo<Evt>,
    ) -> (bool, ButtonEvent)
    where
        Evt: Event,
        Theme: ButtonTheme<Style = Style>,
    {
        if self.state.is_disabled() {
            return (false, ButtonEvent::Nothing);
        }

        let bounds = self.off_bounds(off);

        let new_state = match event.mouse_relate_to(bounds) {
            MouseRelation::None | MouseRelation::Elswhere => {
                return (false, ButtonEvent::Nothing);
            }
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

        let res = match event.get_kind() {
            EventKind::MousePress(b) if self.react.contains(b.into()) => {
                (true, ButtonEvent::Clicked(b))
            }
            EventKind::MouseRelease(b) if self.react.contains(b.into()) => {
                (true, ButtonEvent::Nothing)
            }
            _ => (false, ButtonEvent::Nothing),
        };

        if self.state != new_state
            && theme.is_different(&self.style, self.state, new_state)
        {
            shell.request_redraw();
        }
        self.state = new_state;

        res
    }

    pub fn draw<Rend, Theme>(
        &mut self,
        off: Vec2<f32>,
        theme: &Theme,
        renderer: &mut Rend,
        draw_child: impl FnOnce(&mut Rend),
    ) where
        Rend: QuadRenderer,
        Theme: ButtonTheme<Style = Style>,
    {
        self.draw_direct(off, theme, renderer);
        draw_child(renderer);
    }

    pub fn draw_direct<Rend, Theme>(
        &mut self,
        off: Vec2<f32>,
        theme: &Theme,
        renderer: &mut Rend,
    ) where
        Rend: QuadRenderer,
        Theme: ButtonTheme<Style = Style>,
    {
        self.style.update();
        if let Some(a) = theme.appereance(&self.style, self.state) {
            let bounds = self.off_bounds(off);
            renderer.draw_border(bounds, a.border, a.background);
        }
    }
}
