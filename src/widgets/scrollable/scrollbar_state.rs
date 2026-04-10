#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollbarState {
    Disabled,
    Hidden,
    Visible,
}

impl ScrollbarState {
    pub fn enabled(&self) -> bool {
        !matches!(self, ScrollbarState::Disabled)
    }
}
