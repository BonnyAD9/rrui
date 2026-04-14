use crate::{
    Color,
    widgets::{ButtonState, ContainerAppereance},
};

pub trait ButtonTheme {
    type Style;

    fn appereance(
        &self,
        style: &Self::Style,
        state: ButtonState,
    ) -> Option<ContainerAppereance>;

    fn foreground(
        &self,
        style: &Self::Style,
        state: ButtonState,
    ) -> Option<Color>;

    fn is_different(
        &self,
        style: &Self::Style,
        a: ButtonState,
        b: ButtonState,
    ) -> bool;
}
