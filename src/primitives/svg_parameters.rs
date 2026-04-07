use crate::{Angle, Color};

#[derive(Debug, Clone, Copy)]
pub struct SvgParameters {
    pub color: Option<Color>,
    pub rotation: Angle,
    pub opacity: f32,
}

impl SvgParameters {
    pub fn new(
        color: impl Into<Color>,
        rotation: impl Into<Angle>,
        opacity: f32,
    ) -> Self {
        Self {
            color: color.into().into(),
            rotation: rotation.into(),
            opacity,
        }
    }

    pub fn colored(color: impl Into<Color>) -> Self {
        Self::new(color, Angle::default(), 1.)
    }

    pub fn rotated(rotation: impl Into<Angle>) -> Self {
        Self {
            rotation: rotation.into(),
            ..Default::default()
        }
    }
}

impl Default for SvgParameters {
    fn default() -> Self {
        Self {
            color: Default::default(),
            rotation: Default::default(),
            opacity: 1.,
        }
    }
}
