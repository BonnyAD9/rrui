use crate::widgets::{
    ButtonState, ButtonTheme, ContainerAppereance, ScrollbarSizes, ThumbTheme,
    TrackTheme, scrollbar::scrollbar_style::ScrollbarStyle,
};

pub trait ScrollbarTheme: ButtonTheme + ThumbTheme + TrackTheme {
    type Style: ScrollbarStyle<
            <Self as ButtonTheme>::Style,
            <Self as TrackTheme>::Style,
            <Self as ThumbTheme>::Style,
        >;

    fn sizes(&self, style: &<Self as ScrollbarTheme>::Style)
    -> ScrollbarSizes;
}
