use crate::{
    Color,
    widgets::{ContainerAppereance, InputState},
};

pub trait InputTheme {
    type Style;

    fn appereance(
        &self,
        style: &Self::Style,
        state: InputState,
    ) -> Option<ContainerAppereance>;

    fn foreground(
        &self,
        style: &Self::Style,
        state: InputState,
        requested: Option<Color>,
    ) -> Color;

    fn selection(&self, style: &Self::Style, state: InputState) -> Color;

    fn cursor(&self, style: &Self::Style, state: InputState) -> Color;

    fn is_different(
        &self,
        style: &Self::Style,
        a: InputState,
        b: InputState,
    ) -> bool;
}
