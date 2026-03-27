use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct EventFlags : u32 {
        const WINDOW = 0x1;
        const KEYBOARD = 0x2;
        const MOUSE = 0x4;
        const INPUT = 0x8;
        const OTHER = 0x10;
        const FOR_WIDGETS = 0x20;
    }
}
