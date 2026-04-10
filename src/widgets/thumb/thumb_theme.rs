use minlin::Padding;

use crate::{
    Orientation,
    widgets::{ContainerAppereance, ThumbState},
};

pub trait ThumbTheme {
    type Style;

    fn appereance(
        &self,
        style: &Self::Style,
        state: ThumbState,
        orientation: Orientation,
    ) -> Option<ContainerAppereance>;

    fn is_different(
        &self,
        style: &Self::Style,
        a: ThumbState,
        b: ThumbState,
    ) -> bool;

    fn padding(
        &self,
        style: &Self::Style,
        orientation: Orientation,
    ) -> Padding<f32>;
}
