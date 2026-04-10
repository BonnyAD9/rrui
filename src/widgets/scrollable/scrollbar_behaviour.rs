#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollbarBehaviour {
    Disabled,
    Hidden,
    Visible,
}

impl ScrollbarBehaviour {
    pub fn enabled(&self) -> bool {
        !matches!(self, ScrollbarBehaviour::Disabled)
    }

    pub fn visible(&self) -> bool {
        matches!(self, Self::Visible)
    }
}
