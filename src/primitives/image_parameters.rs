use crate::{Angle, ImageFilter, Radius};

#[derive(Debug, Clone, Copy)]
pub struct ImageParameters {
    pub filter: ImageFilter,
    pub rotation: Angle,
    pub border_radius: Radius,
    pub opacity: f32,
    pub snap: bool,
}

impl ImageParameters {
    pub fn round_rotated(
        border_radius: impl Into<Radius>,
        rotation: Angle,
    ) -> Self {
        Self {
            filter: ImageFilter::default(),
            rotation,
            border_radius: border_radius.into(),
            opacity: 1.,
            snap: false,
        }
    }

    pub fn new() -> Self {
        Self::round_rotated(Radius::square(), Angle::default())
    }

    pub fn round(radius: impl Into<Radius>) -> Self {
        Self::round_rotated(radius, Angle::default())
    }

    pub fn rotated(rotation: Angle) -> Self {
        Self::round_rotated(Radius::square(), rotation)
    }
}

impl Default for ImageParameters {
    fn default() -> Self {
        Self {
            filter: Default::default(),
            rotation: Default::default(),
            border_radius: Default::default(),
            opacity: 1.,
            snap: Default::default(),
        }
    }
}
