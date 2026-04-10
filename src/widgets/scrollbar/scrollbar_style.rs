pub trait ScrollbarStyle {
    type ButtonStyle;
    type TrackStyle;
    type ThumbStyle;

    fn button_style(&self) -> Self::ButtonStyle;

    fn track_style(&self) -> Self::TrackStyle;

    fn thumb_style(&self) -> Self::ThumbStyle;
}
