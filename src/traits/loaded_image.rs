use minlin::Vec2;

pub trait LoadedImage {
    fn size(&self) -> Vec2<u32>;
}
