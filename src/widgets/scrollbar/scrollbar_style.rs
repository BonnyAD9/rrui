pub trait ScrollbarStyle<Button, Track, Thumb> {
    fn button_style(&self) -> Button;

    fn track_style(&self) -> Track;

    fn thumb_style(&self) -> Thumb;
}
