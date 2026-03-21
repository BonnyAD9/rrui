mod event_type;
mod modifiers;
mod mouse_button;
mod scroll_delta;

use std::fmt::Debug;

use smol_str::SmolStr;

pub use self::{
    event_type::*, modifiers::*, mouse_button::*, scroll_delta::*,
};

pub trait Event: Debug {
    fn get_type(&self) -> EventType;

    fn is_window(&self) -> bool {
        matches!(
            self.get_type(),
            EventType::Resize(_)
                | EventType::CloseRequest
                | EventType::Focus(_)
                | EventType::ScaleFactorChange(_)
                | EventType::RedrawRequest
        )
    }

    fn is_keyboard(&self) -> bool {
        matches!(
            self.get_type(),
            EventType::KeyPress
                | EventType::KeyRelease
                | EventType::ModifiersChange(_)
        )
    }

    fn is_mouse(&self) -> bool {
        matches!(
            self.get_type(),
            EventType::MouseMove(_)
                | EventType::MousePress(_)
                | EventType::MouseRelease(_)
                | EventType::MouseScroll(_)
        )
    }

    fn is_input(&self) -> bool {
        matches!(
            self.get_type(),
            EventType::KeyPress
                | EventType::KeyRelease
                | EventType::ModifiersChange(_)
                | EventType::MouseMove(_)
                | EventType::MousePress(_)
                | EventType::MouseRelease(_)
                | EventType::MouseScroll(_)
        )
    }

    fn key_char(&self) -> Option<SmolStr>;

    fn is_other(&self) -> bool {
        matches!(self.get_type(), EventType::Other)
    }
}
