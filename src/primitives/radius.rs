use minlin::Vec4;

#[derive(Debug, Default, Copy, Clone)]
pub struct Radius {
    pub tl: f32,
    pub tr: f32,
    pub br: f32,
    pub bl: f32,
}

impl Radius {
    pub fn new(tl: f32, tr: f32, br: f32, bl: f32) -> Self {
        Self { tl, tr, br, bl }
    }

    pub fn vec(self) -> Vec4<f32> {
        Vec4::new(self.tl, self.tr, self.br, self.bl)
    }

    pub fn same(r: f32) -> Self {
        Self::new(r, r, r, r)
    }

    pub fn square() -> Self {
        Self::default()
    }
}

impl From<Vec4<f32>> for Radius {
    fn from(value: Vec4<f32>) -> Self {
        Self::new(value.x, value.y, value.z, value.w)
    }
}

impl From<(f32, f32, f32, f32)> for Radius {
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> Self {
        Self::new(x, y, z, w)
    }
}

impl From<[f32; 4]> for Radius {
    fn from([x, y, z, w]: [f32; 4]) -> Self {
        Self::new(x, y, z, w)
    }
}

impl From<f32> for Radius {
    fn from(value: f32) -> Self {
        Self::same(value)
    }
}
