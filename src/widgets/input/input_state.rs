use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct InputState : u8 {
        const HOVER = 0x1;
        const DRAG = 0x2;
        const FOCUS = 0x4;
        const VALID = 0x8;
    }
}
