#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ButtonState {
    #[default]
    Normal,
    Hover,
    Pressed,
    Disabled,
}

impl ButtonState {
    pub fn is_disabled(&self) -> bool {
        *self == ButtonState::Disabled
    }
}
