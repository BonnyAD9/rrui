use minlin::{Rect, Vec2};

use crate::event::{Modifiers, MouseState};

#[derive(Debug)]
pub struct Shell<Msg> {
    pub(crate) window_bounds: Rect<f32>,
    pub(crate) redraw: bool,
    pub(crate) relayout: bool,
    pub(crate) mouse_pos: Option<Vec2<f32>>,
    pub(crate) modifiers: Modifiers,
    pub(crate) messages: Vec<Msg>,
    pub(crate) mouse_state: MouseState,
}

impl<Msg> Shell<Msg> {
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

    pub fn msgs(&mut self, i: impl IntoIterator<Item = Msg>) {
        self.messages.extend(i)
    }

    pub fn mouse_state(&self) -> MouseState {
        self.mouse_state
    }
}

impl<Msg> Default for Shell<Msg> {
    fn default() -> Self {
        Self {
            window_bounds: Default::default(),
            redraw: Default::default(),
            relayout: Default::default(),
            mouse_pos: Default::default(),
            modifiers: Default::default(),
            messages: Default::default(),
            mouse_state: Default::default(),
        }
    }
}
