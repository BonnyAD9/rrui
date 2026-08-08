use minlin::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrollDelta {
    Lines(Vec2<f32>),
    Pixels(Vec2<f32>),
}

impl ScrollDelta {
    pub fn swap(&mut self) {
        match self {
            ScrollDelta::Lines(v) => v.swap(),
            ScrollDelta::Pixels(v) => v.swap(),
        }
    }

    pub fn to_pixels(&self, line: f32) -> Vec2<f32> {
        match self {
            ScrollDelta::Lines(l) => *l * line,
            ScrollDelta::Pixels(p) => *p,
        }
    }
}
