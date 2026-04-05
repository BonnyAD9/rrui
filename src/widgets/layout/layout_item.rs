use crate::Size;

#[derive(Debug, Copy, Clone)]
pub struct LayoutItem<W> {
    pub widget: W,
    pub size: Size,
}

impl<W> LayoutItem<W> {
    pub fn new(size: impl Into<Size>, widget: W) -> Self {
        Self {
            size: size.into(),
            widget,
        }
    }
}
