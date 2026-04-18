use std::{fmt::Debug, mem};

use bitvec::{BitArr, array::BitArray};
use minlin::{Rect, Vec2};

use crate::{
    CellWidget, RefVariableIn, RefVariableOut, ShellProxy, VariableIn,
    VariableOut,
    event::{KeyCode, Modifiers, MouseState},
    new_ref_variable, new_variable,
};

pub struct Shell<Rend, Msg, Evt, Theme> {
    pub(crate) window_bounds: Rect<f32>,
    pub(crate) proxy: ShellProxy,
    pub(crate) mouse_pos: Option<Vec2<f32>>,
    pub(crate) modifiers: Modifiers,
    pub(crate) messages: Vec<Msg>,
    pub(crate) mouse_state: MouseState,
    pub(crate) focus_target: Option<CellWidget<Rend, Msg, Evt, Theme>>,
    pub(crate) drag_capture: Option<CellWidget<Rend, Msg, Evt, Theme>>,
    pub(crate) evt_id: u64,
    pub(crate) keyboard_state: BitArr!(for KeyCode::MAX_VALUE),
}

impl<Rend, Msg, Evt, Theme> Shell<Rend, Msg, Evt, Theme> {
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

    pub fn make_ref_variable<T>(
        &self,
        value: impl Into<T>,
    ) -> (RefVariableIn<T>, RefVariableOut<T>) {
        new_ref_variable(self.proxy.clone(), value.into())
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

    pub fn capture_drag(&mut self) {
        if let Some(w) = &self.focus_target {
            self.drag_capture = Some(w.clone());
        }
    }

    pub fn with_focus<T>(
        &mut self,
        target: CellWidget<Rend, Msg, Evt, Theme>,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T {
        let old = self.replace_focus(Some(target));
        let res = f(self);
        self.replace_focus(old);
        res
    }

    pub fn is_pressed(&self, key: KeyCode) -> bool {
        if key == KeyCode::Unknown {
            return false;
        }
        self.keyboard_state[key.value()]
    }

    fn replace_focus(
        &mut self,
        target: Option<CellWidget<Rend, Msg, Evt, Theme>>,
    ) -> Option<CellWidget<Rend, Msg, Evt, Theme>> {
        mem::replace(&mut self.focus_target, target)
    }

    pub fn get_id(&self) -> u64 {
        self.evt_id
    }

    pub(crate) fn inc_id(&mut self) {
        self.evt_id = self.evt_id.wrapping_add(1);
    }
}

impl<Rend, Msg: Debug, Evt, Theme> Debug for Shell<Rend, Msg, Evt, Theme> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Shell")
            .field("window_bounds", &self.window_bounds)
            .field("proxy", &self.proxy)
            .field("mouse_pos", &self.mouse_pos)
            .field("modifiers", &self.modifiers)
            .field("messages", &self.messages)
            .field("mouse_state", &self.mouse_state)
            .field("focus_target", &self.focus_target.is_some())
            .finish()
    }
}

impl<Rend, Msg, Evt, Theme> Default for Shell<Rend, Msg, Evt, Theme> {
    fn default() -> Self {
        Self {
            window_bounds: Default::default(),
            proxy: Default::default(),
            mouse_pos: Default::default(),
            modifiers: Default::default(),
            messages: Default::default(),
            mouse_state: Default::default(),
            focus_target: None,
            drag_capture: None,
            keyboard_state: BitArray::ZERO,
            evt_id: 0,
        }
    }
}
