use minlin::Vec2;

pub trait Window {
    type Config: Default;

    fn size(&self) -> Vec2<u32>;
}
