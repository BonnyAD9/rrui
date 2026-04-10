use std::ops::{Range, RangeInclusive};

use minlin::{Rect, RectExt, Vec2};

use crate::{
    Orientation, QuadRenderer, Shell,
    event::{Event, EventInfo, EventKind, MouseRelation, MouseState},
    widgets::{
        ButtonEvent, ScrollbarTheme, ThumbEvent, ThumbState, ThumbTheme,
    },
};

#[derive(Debug, Clone)]
pub struct PartThumb<Style> {
    pub style: Style,
    pub orientation: Orientation,
    pub range: Range<f32>,
    state: ThumbState,
    bounds: Rect<f32>,
}

impl<Style> PartThumb<Style> {
    pub const REACT: MouseState = MouseState::LEFT;

    pub fn new(style: Style, orientation: Orientation) -> Self {
        Self {
            style,
            orientation,
            state: ThumbState::Normal,
            range: 0.0..0.,
            bounds: Rect::default(),
        }
    }

    pub fn bounds(&self) -> Rect<f32> {
        self.bounds
    }

    pub fn off_bounds(&self, off: Vec2<f32>) -> Rect<f32> {
        let mut res = self.bounds;
        res.move_to(self.bounds.pos() + off);
        res
    }

    pub fn size(&self) -> f32 {
        self.orientation.component(self.bounds.size())
    }

    pub fn layout_direct(&mut self, bounds: Rect<f32>, range: Range<f32>) {
        self.range = range;
        self.bounds = bounds;
    }

    pub fn reposition_by(&mut self, off: Vec2<f32>) {
        self.bounds.set_pos(self.bounds.pos() + off);
        match self.orientation {
            Orientation::Horizontal => {
                self.range.start += off.x;
                self.range.end += off.x;
            }
            Orientation::Vertical => {
                self.range.start += off.y;
                self.range.end += off.y;
            }
        }
    }

    pub fn event<Msg, Evt, Theme>(
        &mut self,
        off: Vec2<f32>,
        shell: &mut Shell<Msg>,
        theme: &Theme,
        event: &EventInfo<Evt>,
    ) -> (bool, ThumbEvent)
    where
        Evt: Event,
        Theme: ThumbTheme<Style = Style>,
    {
        let bounds = self.off_bounds(off);

        let (new_state, res) = match event.mouse_relate_to(bounds) {
            MouseRelation::None | MouseRelation::Elswhere => {
                return (false, ThumbEvent::Nothing);
            }
            MouseRelation::Move => {
                if shell.mouse_state().intersects(Self::REACT) {
                    if let Some(pos) = shell.mouse_pos() {
                        self.drag(bounds, pos)
                    } else {
                        (self.state, (true, ThumbEvent::Nothing))
                    }
                } else {
                    (ThumbState::Hover, (true, ThumbEvent::Nothing))
                }
            }
            MouseRelation::Enter | MouseRelation::Hover => {
                if shell.mouse_state().intersects(Self::REACT) {
                    if let Some(pos) = shell.mouse_pos() {
                        self.press(bounds, pos)
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

    pub fn drag_from<Msg, Theme>(
        &mut self,
        pos: f32,
        shell: &mut Shell<Msg>,
        theme: &Theme,
    ) where
        Theme: ThumbTheme<Style = Style>,
    {
        let old_state = self.state;
        self.state = ThumbState::Dragging(0.);
        self.drag(self.bounds, Vec2::new(pos, pos));
        if theme.is_different(&self.style, old_state, self.state) {
            shell.request_redraw();
        }
    }

    pub fn move_to(&mut self, pos: f32) {
        self.bounds.x = pos.clamp(self.range.start, self.range.end);
    }

    fn drag(
        &mut self,
        bounds: Rect<f32>,
        pos: Vec2<f32>,
    ) -> (ThumbState, (bool, ThumbEvent)) {
        let ThumbState::Dragging(dpos) = self.state else {
            return self.press(bounds, pos);
        };

        let pos = match self.orientation {
            Orientation::Horizontal => {
                let pos = pos.x - dpos;
                self.bounds.x +=
                    pos.clamp(self.range.start, self.range.end) - bounds.x;
                self.bounds.x
            }
            Orientation::Vertical => {
                let pos = pos.y - dpos;
                self.bounds.y +=
                    pos.clamp(self.range.start, self.range.end) - bounds.y;
                self.bounds.y
            }
        };

        (ThumbState::Dragging(dpos), (true, ThumbEvent::Move(pos)))
    }

    fn press(
        &mut self,
        bounds: Rect<f32>,
        pos: Vec2<f32>,
    ) -> (ThumbState, (bool, ThumbEvent)) {
        let dpos = match self.orientation {
            Orientation::Horizontal => pos.x - bounds.x,
            Orientation::Vertical => pos.y - bounds.y,
        };

        (ThumbState::Dragging(dpos), (true, ThumbEvent::Nothing))
    }

    pub fn draw<Rend, Theme>(
        &mut self,
        off: Vec2<f32>,
        theme: &Theme,
        renderer: &mut Rend,
    ) where
        Rend: QuadRenderer,
        Theme: ThumbTheme<Style = Style>,
    {
        let bounds = self.off_bounds(off);
        let a = theme.appereance(&self.style, self.state);
        renderer.draw_border(bounds, a.border, a.background);
    }
}
