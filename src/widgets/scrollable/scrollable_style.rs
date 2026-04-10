use crate::widgets::ScrollbarStyle;

pub trait ScrollableStyle {
    type ScrollbarStyle: ScrollbarStyle;

    fn scrollbar_style(&self) -> Self::ScrollbarStyle;
}
