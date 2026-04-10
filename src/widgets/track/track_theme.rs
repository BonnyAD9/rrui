use crate::widgets::{ContainerAppereance, TrackState};

pub trait TrackTheme {
    type Style;

    fn appereance(
        &self,
        style: &Self::Style,
        state: TrackState,
    ) -> Option<ContainerAppereance>;

    fn is_different(
        &self,
        style: &Self::Style,
        a: TrackState,
        b: TrackState,
    ) -> bool;
}
