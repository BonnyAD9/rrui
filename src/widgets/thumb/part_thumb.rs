use minlin::{Rect, Vec2};

use crate::{
    Orientation, QuadRenderer, Shell,
    event::{Event, EventInfo, EventKind, MouseRelation, MouseState},
    widgets::{ThumbEvent, ThumbLayout, ThumbState, ThumbTheme},
};

#[derive(Debug, Clone)]
pub struct PartThumb<Style> {
    pub style: Style,
    state: ThumbState,
    next_state: ThumbState,
}

impl<Style> PartThumb<Style> {
    pub const REACT: MouseState = MouseState::LEFT;

    pub fn new(style: Style) -> Self {
        Self {
            style,
            state: ThumbState::Normal,
            next_state: ThumbState::Normal,
        }
    }

    pub fn track_hover_start<Rend, Msg, Evt, Theme>(
        &mut self,
        shell: &mut Shell<Rend, Msg, Evt, Theme>,
        theme: &Theme,
    ) where
        Theme: ThumbTheme<Style = Style>,
    {
        self.next_state = ThumbState::TrackHover;
        if self.state != ThumbState::Normal {
            return;
        }
        self.state = ThumbState::TrackHover;
        if theme.is_different(
            &self.style,
            ThumbState::Normal,
            ThumbState::TrackHover,
        ) {
            shell.request_redraw();
        }
    }

    pub fn track_hover_end<Rend, Msg, Evt, Theme>(
        &mut self,
        shell: &mut Shell<Rend, Msg, Evt, Theme>,
        theme: &Theme,
    ) where
        Theme: ThumbTheme<Style = Style>,
    {
        self.next_state = ThumbState::Normal;
        if self.state != ThumbState::TrackHover {
            return;
        }
        self.state = ThumbState::Normal;
        if theme.is_different(
            &self.style,
            ThumbState::TrackHover,
            ThumbState::Normal,
        ) {
            shell.request_redraw();
        }
    }

    pub fn set_track_hover<Rend, Msg, Evt, Theme>(
        &mut self,
        hover: bool,
        shell: &mut Shell<Rend, Msg, Evt, Theme>,
        theme: &Theme,
    ) where
        Theme: ThumbTheme<Style = Style>,
    {
        if hover {
            self.track_hover_start(shell, theme);
        } else {
            self.track_hover_end(shell, theme);
        }
    }

    pub fn is_dragging(&self) -> bool {
        matches!(self.state, ThumbState::Dragging(_))
    }

    pub fn event<Rend, Msg, Evt, Theme>(
        &mut self,
        layout: &ThumbLayout,
        shell: &mut Shell<Rend, Msg, Evt, Theme>,
        theme: &Theme,
        event: &EventInfo<Evt>,
    ) -> (bool, ThumbEvent)
    where
        Evt: Event,
        Theme: ThumbTheme<Style = Style>,
    {
        let (new_state, res) = match event.mouse_relate_to(layout.bounds) {
            MouseRelation::None => {
                return (false, ThumbEvent::Nothing);
            }
            MouseRelation::Elswhere => {
                if self.state == ThumbState::Normal {
                    return (false, ThumbEvent::Nothing);
                }
                if !event.is_drag_capture() {
                    (self.next_state, (false, ThumbEvent::Nothing))
                } else {
                    match event.get_kind() {
                        EventKind::MouseMove(_) => {
                            if let Some(mpos) = shell.mouse_pos() {
                                self.drag(layout, shell, mpos)
                            } else {
                                return (true, ThumbEvent::Nothing);
                            }
                        }
                        EventKind::MouseRelease(_) => {
                            (self.next_state, (true, ThumbEvent::Nothing))
                        }
                        _ => return (false, ThumbEvent::Nothing),
                    }
                }
            }
            MouseRelation::Move => {
                if shell.mouse_state().intersects(Self::REACT) {
                    if let Some(mpos) = shell.mouse_pos() {
                        self.drag(layout, shell, mpos)
                    } else {
                        return (true, ThumbEvent::Nothing);
                    }
                } else {
                    (ThumbState::Hover, (false, ThumbEvent::Nothing))
                }
            }
            MouseRelation::Enter | MouseRelation::Hover => {
                if shell.mouse_state().intersects(Self::REACT) {
                    if !self.is_dragging()
                        && let Some(mpos) = shell.mouse_pos()
                    {
                        self.press(layout, shell, mpos)
                    } else {
                        return (true, ThumbEvent::Nothing);
                    }
                } else {
                    (ThumbState::Hover, (false, ThumbEvent::Nothing))
                }
            }
            MouseRelation::Leave => {
                if matches!(self.state, ThumbState::Dragging(_)) {
                    return (false, ThumbEvent::Nothing);
                }
                (self.next_state, (false, ThumbEvent::Nothing))
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

    pub fn start_drag<Rend, Msg, Evt, Theme>(
        &mut self,
        dpos: f32,
        shell: &mut Shell<Rend, Msg, Evt, Theme>,
    ) {
        self.state = ThumbState::Dragging(dpos);
        shell.capture_drag();
        shell.request_redraw();
    }

    fn drag<Rend, Msg, Evt, Theme>(
        &mut self,
        layout: &ThumbLayout,
        shell: &mut Shell<Rend, Msg, Evt, Theme>,
        mpos: Vec2<f32>,
    ) -> (ThumbState, (bool, ThumbEvent)) {
        let ThumbState::Dragging(dpos) = self.state else {
            return self.press(layout, shell, mpos);
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

    fn press<Rend, Msg, Evt, Theme>(
        &mut self,
        layout: &ThumbLayout,
        shell: &mut Shell<Rend, Msg, Evt, Theme>,
        pos: Vec2<f32>,
    ) -> (ThumbState, (bool, ThumbEvent)) {
        let dpos = match layout.orientation {
            Orientation::Horizontal => pos.x - layout.bounds.x,
            Orientation::Vertical => pos.y - layout.bounds.y,
        };

        shell.capture_drag();
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
