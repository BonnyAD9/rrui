use crate::widgets::{ContainerAppereance, ThumbState};

pub trait ThumbTheme {
    type Style;

    fn appereance(
        &self,
        style: &Self::Style,
        state: ThumbState,
    ) -> ContainerAppereance;

    fn is_different(
        &self,
        style: &Self::Style,
        a: ThumbState,
        b: ThumbState,
    ) -> bool;
}
