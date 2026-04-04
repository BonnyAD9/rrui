use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Default, Clone, Copy)]
    pub struct LayoutFlags : u32 {
        const WIDGET_MODIFIED = 0x1;
    }
}
