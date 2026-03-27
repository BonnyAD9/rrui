use minlin::{Rect, Vec2};

use crate::event::Modifiers;

#[derive(Debug, Default)]
pub struct Shell {
    pub(crate) window_bounds: Rect<f32>,
    pub(crate) redraw: bool,
    pub(crate) relayout: bool,
    pub(crate) mouse_pos: Option<Vec2<f32>>,
    pub(crate) modifiers: Modifiers,
}

impl Shell {
    pub fn request_redraw(&mut self) {
        self.redraw = true;
    }

    pub fn redraw_requested(&self) -> bool {
        self.redraw
    }

    pub fn request_relayout(&mut self) {
        self.relayout = true;
    }

    pub fn relayout_requested(&self) -> bool {
        self.relayout
    }

    pub fn mouse_pos(&self) -> Option<Vec2<f32>> {
        self.mouse_pos
    }

    pub fn modifiers(&self) -> Modifiers {
        self.modifiers
    }
}
