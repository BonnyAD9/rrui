#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Angle(f32);

impl Angle {
    pub fn from_radians(rad: f32) -> Self {
        Self(rad)
    }

    pub fn from_degrees(deg: f32) -> Self {
        const MUL: f32 = std::f32::consts::PI / 180.;
        Self(deg * MUL)
    }

    pub fn radians(self) -> f32 {
        self.0
    }

    pub fn degrees(self) -> f32 {
        const MUL: f32 = 180. / std::f32::consts::PI;
        self.0 * MUL
    }
}
