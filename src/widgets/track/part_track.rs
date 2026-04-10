use minlin::{Rect, RectExt, Vec2};

use crate::{
    Orientation, QuadRenderer, Shell,
    event::{Event, EventInfo, EventKind, MouseRelation, MouseState},
    widgets::{PartThumb, TrackEvent, TrackState, TrackTheme},
};

#[derive(Debug, Clone, Copy)]
pub struct PartTrack<Style> {
    pub style: Style,
    state: TrackState,
}

impl<Style> PartTrack<Style> {
    pub const REACT: MouseState = MouseState::LEFT;

    pub fn new(style: Style) -> Self {
        Self {
            style,
            state: TrackState::Normal,
        }
    }

    pub fn event<Msg, Evt, Theme>(
        &mut self,
        bounds: Rect<f32>,
        shell: &mut Shell<Msg>,
        theme: &Theme,
        event: &EventInfo<Evt>,
        orientation: Orientation,
    ) -> (bool, TrackEvent)
    where
        Evt: Event,
        Theme: TrackTheme<Style = Style>,
    {
        let mut new_state = match event.mouse_relate_to(bounds) {
            MouseRelation::None | MouseRelation::Elswhere => {
                return (false, TrackEvent::Nothing);
            }
            MouseRelation::Enter
            | MouseRelation::Move
            | MouseRelation::Hover => TrackState::Hover,
            MouseRelation::Leave => TrackState::Normal,
        };

        let res = match event.get_kind() {
            EventKind::MousePress(b) if Self::REACT.contains(b.into()) => {
                if let Some(pos) = shell.mouse_pos() {
                    new_state = TrackState::Normal;
                    match orientation {
                        Orientation::Horizontal => {
                            (true, TrackEvent::PressAt(pos.x))
                        }
                        Orientation::Vertical => {
                            (false, TrackEvent::PressAt(pos.y))
                        }
                    }
                } else {
                    (true, TrackEvent::Nothing)
                }
            }
            _ => (false, TrackEvent::Nothing),
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
        bounds: impl FnOnce() -> Rect<f32>,
        theme: &Theme,
        renderer: &mut Rend,
        orientation: Orientation,
    ) where
        Rend: QuadRenderer,
        Theme: TrackTheme<Style = Style>,
    {
        if let Some(a) = theme.appereance(&self.style, self.state, orientation)
        {
            let bounds = bounds() + theme.padding(&self.style, orientation);
            renderer.draw_border(bounds, a.border, a.background);
        }
    }
}
