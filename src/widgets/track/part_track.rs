use minlin::{Rect, RectExt, Vec2};

use crate::{
    Orientation, QuadRenderer, Shell,
    event::{Event, EventInfo, EventKind, MouseRelation, MouseState},
    widgets::{PartThumb, TrackEvent, TrackState, TrackTheme},
};

#[derive(Debug, Clone, Copy)]
pub struct PartTrack<Style> {
    pub style: Style,
    pub orientation: Orientation,
    pub bounds: Rect<f32>,
    state: TrackState,
}

impl<Style> PartTrack<Style> {
    pub const REACT: MouseState = MouseState::LEFT;

    pub fn new(style: Style, orientation: Orientation) -> Self {
        Self {
            style,
            orientation,
            bounds: Rect::default(),
            state: TrackState::Normal,
        }
    }

    pub fn off_bounds(&self, off: Vec2<f32>) -> Rect<f32> {
        let mut res = self.bounds;
        res.move_to(self.bounds.pos() + off);
        res
    }

    pub fn reposition(&mut self, pos: Vec2<f32>) {
        self.bounds.set_pos(pos);
    }

    pub fn event<Msg, Evt, Theme>(
        &mut self,
        off: Vec2<f32>,
        shell: &mut Shell<Msg>,
        theme: &Theme,
        event: &EventInfo<Evt>,
    ) -> (bool, TrackEvent)
    where
        Evt: Event,
        Theme: TrackTheme<Style = Style>,
    {
        let bounds = self.off_bounds(off);

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
                    match self.orientation {
                        Orientation::Horizontal => {
                            (true, TrackEvent::PressAt(pos.x - off.x))
                        }
                        Orientation::Vertical => {
                            (false, TrackEvent::PressAt(pos.y - off.y))
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
        off: Vec2<f32>,
        theme: &Theme,
        renderer: &mut Rend,
    ) where
        Rend: QuadRenderer,
        Theme: TrackTheme<Style = Style>,
    {
        if let Some(a) = theme.appereance(&self.style, self.state) {
            let bounds = self.off_bounds(off);
            renderer.draw_border(bounds, a.border, a.background);
        }
    }
}
