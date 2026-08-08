use crate::event::{KeyCode, Modifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EditorMotion {
    Left,
    Right,
    Up,
    Down,
    WordLeft,
    WordRight,
    Home,
    End,
    PageUp,
    PageDown,
    DocumentStart,
    DocumentEnd,
}

impl EditorMotion {
    pub fn from_key(key: KeyCode, mods: Modifiers) -> Option<Self> {
        let res = match key {
            KeyCode::ArrowLeft if mods.ctrl() => Self::WordLeft,
            KeyCode::ArrowLeft => Self::Left,
            KeyCode::ArrowRight if mods.ctrl() => Self::WordRight,
            KeyCode::ArrowRight => Self::Right,
            KeyCode::ArrowUp => Self::Up,
            KeyCode::ArrowDown => Self::Down,
            KeyCode::Home if mods.ctrl() => Self::DocumentStart,
            KeyCode::Home => Self::Home,
            KeyCode::End if mods.ctrl() => Self::DocumentEnd,
            KeyCode::End => Self::End,
            KeyCode::PageUp => Self::PageUp,
            KeyCode::PageDown => Self::PageDown,
            _ => return None,
        };
        Some(res)
    }
}
