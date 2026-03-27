use bitflags::bitflags;

use crate::event::MouseButton;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct MouseState: u32 {
        const LEFT = 0x1;
        const RIGHT = 0x2;
        const MIDDLE = 0x4;
        const BACK = 0x8;
        const FORWARD = 0x10;
        const OTHER = 0x20;
    }
}

impl MouseState {
    pub fn press(&mut self, btn: MouseButton) {
        *self |= btn.into();
    }

    pub fn release(&mut self, btn: MouseButton) {
        self.remove(btn.into());
    }
}

impl From<MouseButton> for MouseState {
    fn from(value: MouseButton) -> Self {
        match value {
            MouseButton::Left => Self::LEFT,
            MouseButton::Right => Self::RIGHT,
            MouseButton::Middle => Self::MIDDLE,
            MouseButton::Back => Self::BACK,
            MouseButton::Forward => Self::FORWARD,
            MouseButton::Other(n) => {
                Self::from_bits_retain(Self::OTHER.bits() << n)
            }
        }
    }
}
