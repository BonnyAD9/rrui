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
