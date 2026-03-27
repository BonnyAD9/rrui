mod event_ctx;
mod event_flags;
mod event_info;
mod event_kind;
mod modifiers;
mod mouse_button;
mod mouse_relation;
mod mouse_state;
mod scroll_delta;

use std::fmt::Debug;

use smol_str::SmolStr;

pub use self::{
    event_ctx::*, event_flags::*, event_info::*, event_kind::*, modifiers::*,
    mouse_button::*, mouse_relation::*, mouse_state::*, scroll_delta::*,
};

pub trait Event: Debug {
    fn get_kind(&self) -> EventKind;

    fn is_window(&self) -> bool {
        self.get_flags().contains(EventFlags::WINDOW)
    }

    fn is_keyboard(&self) -> bool {
        self.get_flags().contains(EventFlags::KEYBOARD)
    }

    fn is_mouse(&self) -> bool {
        self.get_flags().contains(EventFlags::MOUSE)
    }

    fn is_input(&self) -> bool {
        self.get_flags().contains(EventFlags::INPUT)
    }

    fn is_for_widgets(&self) -> bool {
        self.get_flags().contains(EventFlags::FOR_WIDGETS)
    }

    fn key_char(&self) -> Option<SmolStr>;

    fn get_flags(&self) -> EventFlags {
        match self.get_kind() {
            EventKind::Resize(_)
            | EventKind::WindowFocus(_)
            | EventKind::ScaleFactorChange(_)
            | EventKind::RedrawRequest => EventFlags::WINDOW,
            EventKind::CloseRequest => EventFlags::WINDOW | EventFlags::INPUT,
            EventKind::KeyPress
            | EventKind::KeyRelease
            | EventKind::ModifiersChange(_) => {
                EventFlags::KEYBOARD
                    | EventFlags::INPUT
                    | EventFlags::FOR_WIDGETS
            }
            EventKind::MouseMove(_)
            | EventKind::MousePress(_)
            | EventKind::MouseRelease(_)
            | EventKind::MouseScroll(_) => {
                EventFlags::MOUSE | EventFlags::INPUT | EventFlags::FOR_WIDGETS
            }
            EventKind::MouseLeaveWindow => {
                EventFlags::MOUSE
                    | EventFlags::WINDOW
                    | EventFlags::FOR_WIDGETS
            }
            EventKind::MouseEnterWindow => {
                EventFlags::MOUSE | EventFlags::WINDOW
            }
            EventKind::Other => EventFlags::OTHER,
        }
    }

    fn is_other(&self) -> bool {
        self.get_flags().contains(EventFlags::OTHER)
    }
}
