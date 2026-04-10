use crate::widgets::{ContainerAppereance, ScrollableStyle, ScrollbarTheme};

pub trait ScrollableTheme: ScrollbarTheme {
    type Style: ScrollableStyle;

    fn appereance(
        &self,
        style: &<Self as ScrollableTheme>::Style,
    ) -> Option<ContainerAppereance>;
}
