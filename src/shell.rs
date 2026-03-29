use minlin::{Rect, Vec2};

use crate::{
    ShellProxy, VariableIn, VariableOut,
    event::{Modifiers, MouseState},
    new_variable,
};

#[derive(Debug)]
pub struct Shell<Msg> {
    pub(crate) window_bounds: Rect<f32>,
    pub(crate) proxy: ShellProxy,
    pub(crate) mouse_pos: Option<Vec2<f32>>,
    pub(crate) modifiers: Modifiers,
    pub(crate) messages: Vec<Msg>,
    pub(crate) mouse_state: MouseState,
}

impl<Msg> Shell<Msg> {
    pub fn request_redraw(&mut self) {
        self.proxy.request_redraw();
    }

    pub fn redraw_requested(&self) -> bool {
        self.proxy.redraw_requested()
    }

    pub(crate) fn reset_redraw(&self) {
        self.proxy.0.redraw.set(false);
    }

    pub fn request_relayout(&mut self) {
        self.proxy.request_relayout();
    }

    pub fn relayout_requested(&self) -> bool {
        self.proxy.relayout_requested()
    }

    pub(crate) fn reset_relayout(&self) {
        self.proxy.0.relayout.set(false);
    }

    pub fn proxy(&self) -> ShellProxy {
        self.proxy.clone()
    }

    pub fn make_variable<T>(
        &self,
        value: impl Into<T>,
    ) -> (VariableIn<T>, VariableOut<T>) {
        new_variable(self.proxy.clone(), value.into())
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
            proxy: Default::default(),
            mouse_pos: Default::default(),
            modifiers: Default::default(),
            messages: Default::default(),
            mouse_state: Default::default(),
        }
    }
}
