#[derive(Debug, Clone, PartialEq, Copy, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Other(u16),
}
