use crate::event::{KeyCode, Modifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EditorEdit<'a> {
    Insert(char),
    Paste(&'a str),
    Enter,
    Indent,
    Unindent,
    Backspace,
    Delete,
}

impl<'a> EditorEdit<'a> {
    pub fn from_key(key: KeyCode, mods: Modifiers) -> Option<Self> {
        let res = match key {
            KeyCode::Enter => Self::Enter,
            KeyCode::Tab if mods.shift() => Self::Unindent,
            KeyCode::Tab => Self::Indent,
            KeyCode::Backspace => Self::Backspace,
            KeyCode::Delete => Self::Delete,
            _ => return None,
        };
        Some(res)
    }
}
