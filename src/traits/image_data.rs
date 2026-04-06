use std::path::Path;

use bytes::Bytes;
use minlin::Vec2;

pub trait ImageData: Clone + 'static {
    fn from_path(path: impl AsRef<Path>) -> Self;

    fn from_data(data: Bytes) -> Self;

    fn from_rgba(size: Vec2<u32>, data: Bytes) -> Self;
}
