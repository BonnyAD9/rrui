use bitflags::bitflags;

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
