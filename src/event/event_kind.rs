use minlin::Vec2;

use crate::event::{Modifiers, MouseButton, ScrollDelta};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum EventKind {
    // Window
    Resize(Vec2<u32>),
    CloseRequest,
    WindowFocus(bool),
    ScaleFactorChange(f32),
    RedrawRequest,

    // Keyboard
    KeyPress,
    KeyRelease,
    ModifiersChange(Modifiers),

    // Mouse
    MouseMove(Vec2<f32>),
    MousePress(MouseButton),
    MouseRelease(MouseButton),
    MouseScroll(ScrollDelta),
    MouseLeaveWindow,
    MouseEnterWindow,

    // Other
    Other,
}
