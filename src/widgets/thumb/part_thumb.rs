use std::ops::{Range, RangeInclusive};

use minlin::{Rect, RectExt, Vec2};

use crate::{
    Orientation, QuadRenderer, Shell,
    event::{Event, EventInfo, EventKind, MouseRelation, MouseState},
    widgets::{
        ButtonEvent, ScrollbarTheme, ThumbEvent, ThumbLayout, ThumbState,
        ThumbTheme,
    },
};

#[derive(Debug, Clone)]
pub struct PartThumb<Style> {
    pub style: Style,
    state: ThumbState,
}

impl<Style> PartThumb<Style> {
    pub const REACT: MouseState = MouseState::LEFT;

    pub fn new(style: Style) -> Self {
        Self {
            style,
            state: ThumbState::Normal,
        }
    }

    pub fn event<Msg, Evt, Theme>(
        &mut self,
        layout: &ThumbLayout,
        shell: &mut Shell<Msg>,
        theme: &Theme,
        event: &EventInfo<Evt>,
    ) -> (bool, ThumbEvent)
    where
        Evt: Event,
        Theme: ThumbTheme<Style = Style>,
    {
        let (new_state, res) = match event.mouse_relate_to(layout.bounds) {
            MouseRelation::None | MouseRelation::Elswhere => {
                return (false, ThumbEvent::Nothing);
            }
            MouseRelation::Move => {
                if shell.mouse_state().intersects(Self::REACT) {
                    if let Some(mpos) = shell.mouse_pos() {
                        self.drag(layout, mpos)
                    } else {
                        (self.state, (true, ThumbEvent::Nothing))
                    }
                } else {
                    (ThumbState::Hover, (true, ThumbEvent::Nothing))
                }
            }
            MouseRelation::Enter | MouseRelation::Hover => {
                if shell.mouse_state().intersects(Self::REACT) {
                    if let Some(mpos) = shell.mouse_pos() {
                        self.press(layout, mpos)
                    } else {
                        (self.state, (true, ThumbEvent::Nothing))
                    }
                } else {
                    (ThumbState::Hover, (true, ThumbEvent::Nothing))
                }
            }
            MouseRelation::Leave => {
                (ThumbState::Normal, (false, ThumbEvent::Nothing))
            }
        };

        if new_state != self.state
            && theme.is_different(&self.style, self.state, new_state)
        {
            shell.request_redraw();
        }
        self.state = new_state;

        res
    }

    pub fn start_drag<Msg>(&mut self, dpos: f32, shell: &mut Shell<Msg>) {
        self.state = ThumbState::Dragging(dpos);
        shell.request_redraw();
    }

    fn drag(
        &mut self,
        layout: &ThumbLayout,
        mpos: Vec2<f32>,
    ) -> (ThumbState, (bool, ThumbEvent)) {
        let ThumbState::Dragging(dpos) = self.state else {
            return self.press(layout, mpos);
        };

        let pos = match layout.orientation {
            Orientation::Horizontal => {
                (mpos.x - dpos).clamp(layout.range.start, layout.range.end)
            }
            Orientation::Vertical => {
                (mpos.y - dpos).clamp(layout.range.start, layout.range.end)
            }
        };

        (ThumbState::Dragging(dpos), (true, ThumbEvent::Move(pos)))
    }

    fn press(
        &mut self,
        layout: &ThumbLayout,
        pos: Vec2<f32>,
    ) -> (ThumbState, (bool, ThumbEvent)) {
        let dpos = match layout.orientation {
            Orientation::Horizontal => pos.x - layout.bounds.x,
            Orientation::Vertical => pos.y - layout.bounds.y,
        };

        (ThumbState::Dragging(dpos), (true, ThumbEvent::Nothing))
    }

    pub fn draw<Rend, Theme>(
        &mut self,
        bounds: impl FnOnce() -> Rect<f32>,
        theme: &Theme,
        renderer: &mut Rend,
        orientation: Orientation,
    ) where
        Rend: QuadRenderer,
        Theme: ThumbTheme<Style = Style>,
    {
        if let Some(a) = theme.appereance(&self.style, self.state, orientation)
        {
            let bounds = bounds() + theme.padding(&self.style, orientation);
            renderer.draw_border(bounds, a.border, a.background);
        }
    }
}
