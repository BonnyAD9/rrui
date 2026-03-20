mod event_type;
mod modifiers;
mod mouse_button;
mod scroll_delta;

use std::fmt::Debug;

use minlin::Vec2;
use smol_str::SmolStr;
use winit::event::{ElementState, KeyEvent, WindowEvent};

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

impl Event for WindowEvent {
    fn get_type(&self) -> EventType {
        match self {
            Self::Resized(s) => {
                EventType::Resize(Vec2::new(s.width, s.height))
            }
            Self::CloseRequested => EventType::CloseRequest,
            Self::Focused(f) => EventType::Focus(*f),
            Self::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => EventType::KeyPress,
            Self::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Released,
                        ..
                    },
                ..
            } => EventType::KeyRelease,
            Self::ModifiersChanged(m) => {
                EventType::ModifiersChange((*m).into())
            }
            Self::CursorMoved { position, .. } => {
                EventType::MouseMove(Vec2::new(position.x, position.y).cast())
            }
            Self::MouseWheel { delta, .. } => {
                EventType::MouseScroll((*delta).into())
            }
            Self::MouseInput {
                state: ElementState::Pressed,
                button,
                ..
            } => EventType::MousePress((*button).into()),
            Self::MouseInput {
                state: ElementState::Released,
                button,
                ..
            } => EventType::MouseRelease((*button).into()),
            Self::ScaleFactorChanged { scale_factor, .. } => {
                EventType::ScaleFactorChange(*scale_factor as f32)
            }
            Self::RedrawRequested => EventType::RedrawRequest,
            _ => EventType::Other,
        }
    }

    fn key_char(&self) -> Option<SmolStr> {
        match self {
            Self::KeyboardInput {
                event: KeyEvent { text, .. },
                ..
            } => (*text).clone(),
            _ => None,
        }
    }

    fn is_window(&self) -> bool {
        matches!(
            self,
            Self::Resized(_)
                | Self::Moved(_)
                | Self::CloseRequested
                | Self::Destroyed
                | Self::Focused(_)
                | Self::CursorEntered { .. }
                | Self::CursorLeft { .. }
                | Self::ScaleFactorChanged { .. }
                | Self::ThemeChanged(_)
                | Self::Occluded(_)
                | Self::RedrawRequested
        )
    }

    fn is_keyboard(&self) -> bool {
        matches!(self, Self::KeyboardInput { .. } | Self::ModifiersChanged(_))
    }

    fn is_mouse(&self) -> bool {
        matches!(
            self,
            Self::DroppedFile(_)
                | Self::HoveredFile(_)
                | Self::HoveredFileCancelled
                | Self::CursorMoved { .. }
                | Self::CursorEntered { .. }
                | Self::CursorLeft { .. }
                | Self::MouseWheel { .. }
                | Self::MouseInput { .. }
        )
    }

    fn is_input(&self) -> bool {
        matches!(
            self,
            Self::CloseRequested
                | Self::KeyboardInput { .. }
                | Self::ModifiersChanged(_)
                | Self::Ime(_)
                | Self::CursorMoved { .. }
                | Self::MouseWheel { .. }
                | Self::MouseInput { .. }
                | Self::PinchGesture { .. }
                | Self::PanGesture { .. }
                | Self::DoubleTapGesture { .. }
                | Self::RotationGesture { .. }
                | Self::TouchpadPressure { .. }
                | Self::AxisMotion { .. }
                | Self::Touch(..)
        )
    }
}
