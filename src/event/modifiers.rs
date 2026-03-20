use bitflags::bitflags;
use winit::keyboard::ModifiersKeyState;

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Modifiers : u32 {
        const LSHIFT   = 0b000_000_000_001;
        const RSHIFT   = 0b000_000_000_010;
        const SHIFT    = 0b000_000_000_111;
        const LCONTROL = 0b000_000_001_000;
        const RCONTROL = 0b000_000_010_000;
        const CONTROL  = 0b000_000_111_000;
        const LALT     = 0b000_001_000_000;
        const RALT     = 0b000_010_000_000;
        const ALT      = 0b000_111_000_000;
        const LSUPER   = 0b001_000_000_000;
        const RSUPER   = 0b010_000_000_000;
        const SUPER    = 0b111_000_000_000;
    }
}

impl From<winit::event::Modifiers> for Modifiers {
    fn from(value: winit::event::Modifiers) -> Self {
        let mut res = Self::from_bits_truncate(value.state().bits());

        if value.lshift_state() == ModifiersKeyState::Pressed {
            res |= Self::LSHIFT;
        }
        if value.rshift_state() == ModifiersKeyState::Pressed {
            res |= Self::RSHIFT;
        }
        if value.lcontrol_state() == ModifiersKeyState::Pressed {
            res |= Self::LCONTROL;
        }
        if value.rcontrol_state() == ModifiersKeyState::Pressed {
            res |= Self::RCONTROL;
        }
        if value.lalt_state() == ModifiersKeyState::Pressed {
            res |= Self::LALT;
        }
        if value.ralt_state() == ModifiersKeyState::Pressed {
            res |= Self::RALT;
        }
        if value.lsuper_state() == ModifiersKeyState::Pressed {
            res |= Self::LSUPER;
        }
        if value.rsuper_state() == ModifiersKeyState::Pressed {
            res |= Self::RSUPER;
        }

        res
    }
}
