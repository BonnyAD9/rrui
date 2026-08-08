use minlin::{Rect, Vec2};

pub enum Selection {
    Caret(Vec2<f32>),
    Range(Vec<Rect<f32>>),
}
