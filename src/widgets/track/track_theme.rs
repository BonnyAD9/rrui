use minlin::Padding;

use crate::{
    Orientation,
    widgets::{ContainerAppereance, TrackState},
};

pub trait TrackTheme {
    type Style;

    fn appereance(
        &self,
        style: &Self::Style,
        state: TrackState,
        orientation: Orientation,
    ) -> Option<ContainerAppereance>;

    fn is_different(
        &self,
        style: &Self::Style,
        a: TrackState,
        b: TrackState,
    ) -> bool;

    fn padding(
        &self,
        style: &Self::Style,
        orientation: Orientation,
    ) -> Padding<f32>;
}
